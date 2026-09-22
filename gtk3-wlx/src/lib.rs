// Copyright (C) 2026 Martin Brozkeff Malec
// Licensed under the EUPL, Version 1.2.

//! Secondary GTK3 WLX plugin for Double Commander.
//!
//! The host owns the GTK main loop and passes GTK container handles through
//! the Total Commander-compatible WLX ABI. PDF parsing is deliberately kept
//! out of this process: `mutool draw` renders bounded page images in a child
//! process, and GTK loads those images before their private temporary
//! directory is removed.

use std::ffi::{c_char, c_void, CStr, OsStr};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use gtk::prelude::*;

const MAX_PAGES: usize = 128;
const RENDER_DPI: u32 = 120;
const DETECT_STRING: &[u8] = b"EXT=\"PDF\"";
const LICENSE: &[u8] = b"Copyright (C) 2026 Martin Brozkeff Malec; licensed under the EUPL 1.2\0";
const VERSION: &[u8] = b"0.2.0-rust-gtk3\0";
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TempPages {
    directory: PathBuf,
    pages: Vec<PathBuf>,
}

impl Drop for TempPages {
    fn drop(&mut self) {
        // This path was created by us with a unique name; removing it also
        // cleans up partial mutool output after every error path.
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn temporary_directory() -> Result<PathBuf, ()> {
    let root = std::env::temp_dir();
    for _ in 0..16 {
        let nonce = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ())?
            .as_nanos();
        let directory = root.join(format!(
            "doublecmd-pdf-rust-{}-{timestamp}-{nonce}",
            std::process::id()
        ));
        if fs::create_dir(&directory).is_ok() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))
                    .map_err(|_| ())?;
            }
            return Ok(directory);
        }
    }
    Err(())
}

fn page_count(output: &[u8]) -> Option<usize> {
    std::str::from_utf8(output)
        .ok()?
        .lines()
        .find_map(|line| line.strip_prefix("Pages:")?.trim().parse().ok())
}

fn render_pages(file_name: &Path) -> Result<TempPages, ()> {
    let info = Command::new("mutool")
        .arg("info")
        .arg(file_name)
        .output()
        .map_err(|_| ())?;
    if !info.status.success() {
        return Err(());
    }
    let pages = page_count(&info.stdout).ok_or(())?;
    if pages == 0 || pages > MAX_PAGES {
        return Err(());
    }

    let directory = temporary_directory()?;
    let pattern = directory.join("%d.png");
    let status = Command::new("mutool")
        .args(["draw", "-q", "-r"])
        .arg(RENDER_DPI.to_string())
        .args(["-o"])
        .arg(&pattern)
        .arg(file_name)
        .arg(format!("1-{pages}"))
        .status()
        .map_err(|_| ())?;
    if !status.success() {
        return Err(());
    }

    let page_paths: Vec<_> = (1..=pages)
        .map(|page| directory.join(format!("{page}.png")))
        .collect();
    if page_paths.iter().any(|page| !page.is_file()) {
        return Err(());
    }
    Ok(TempPages {
        directory,
        pages: page_paths,
    })
}

fn path_from_c_string(file_to_load: *const c_char) -> Option<PathBuf> {
    // The WLX ABI promises a valid NUL-terminated filename for the duration
    // of the call. The bytes are copied into an owned path before returning.
    // SAFETY: the caller validated the non-null pointer and WLX supplies the
    // required NUL-terminated string for the duration of this call.
    let bytes = unsafe { CStr::from_ptr(file_to_load).to_bytes() };
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        Some(PathBuf::from(OsStr::from_bytes(bytes)))
    }
    #[cfg(not(unix))]
    {
        let text = std::str::from_utf8(bytes).ok()?;
        Some(PathBuf::from(text))
    }
}

fn ensure_gtk_host_state() -> bool {
    if gtk::is_initialized_main_thread() {
        return true;
    }
    if gtk::is_initialized() {
        return false;
    }
    // SAFETY: WLX calls are made by Double Commander on its initialized GTK
    // GUI thread. We only synchronize gtk-rs's bookkeeping with that host;
    // this does not initialize GTK or create a second main loop.
    unsafe { gtk::set_initialized() };
    gtk::is_initialized_main_thread()
}

fn load_plugin(parent: *mut c_void, file_to_load: *const c_char) -> *mut c_void {
    if parent.is_null() || file_to_load.is_null() || !ensure_gtk_host_state() {
        return std::ptr::null_mut();
    }
    let Some(file_name) = path_from_c_string(file_to_load) else {
        return std::ptr::null_mut();
    };
    let Ok(rendered) = render_pages(&file_name) else {
        return std::ptr::null_mut();
    };

    let scroll = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let column = gtk::Box::new(gtk::Orientation::Vertical, 8);
    for page in &rendered.pages {
        let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_file(page) else {
            return std::ptr::null_mut();
        };
        let image = gtk::Image::from_pixbuf(Some(&pixbuf));
        column.pack_start(&image, false, false, 0);
    }
    scroll.add(&column);
    scroll.show_all();

    // SAFETY: the WLX ABI supplies ParentWin as a live GtkContainer. GTK's
    // container takes ownership of the widget reference; the Rust wrapper
    // remains valid until this function returns and the host owns the child.
    unsafe {
        gtk::ffi::gtk_container_add(
            parent.cast::<gtk::ffi::GtkContainer>(),
            scroll.as_ptr().cast::<gtk::ffi::GtkWidget>(),
        );
    }
    scroll.as_ptr().cast()
}

fn guarded<T>(operation: impl FnOnce() -> T) -> Option<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)).ok()
}

/// WLX ABI entry point. `HWND` is `void *` and Linux WLX uses the C calling
/// convention; the host must call this on its GTK GUI thread.
#[no_mangle]
pub extern "C" fn ListLoad(
    parent: *mut c_void,
    file_to_load: *const c_char,
    _show_flags: i32,
) -> *mut c_void {
    guarded(|| load_plugin(parent, file_to_load)).unwrap_or(std::ptr::null_mut())
}

/// WLX ABI entry point. The handle must be one previously returned by
/// [`ListLoad`].
#[no_mangle]
pub extern "C" fn ListCloseWindow(list_window: *mut c_void) {
    let _ = guarded(|| {
        if list_window.is_null() || !ensure_gtk_host_state() {
            return;
        }
        // SAFETY: the host passes back the live GtkWidget handle returned by
        // ListLoad. GTK performs the widget destruction on its GUI thread.
        unsafe { gtk::ffi::gtk_widget_destroy(list_window.cast()) };
    });
}

fn write_detect_string(buffer: &mut [u8]) {
    if buffer.is_empty() {
        return;
    }
    let length = DETECT_STRING.len().min(buffer.len() - 1);
    buffer[..length].copy_from_slice(&DETECT_STRING[..length]);
    buffer[length] = 0;
}

/// WLX ABI entry point. The output is always NUL-terminated when `max_len`
/// is positive, including when the supplied buffer is shorter than the text.
#[no_mangle]
pub extern "C" fn ListGetDetectString(buffer: *mut c_char, max_len: i32) {
    let _ = guarded(|| {
        if buffer.is_null() || max_len <= 0 {
            return;
        }
        // SAFETY: WLX supplies a writable buffer of at least max_len bytes.
        // The slice is used only for this call and write_detect_string never
        // writes beyond its declared length.
        let buffer =
            unsafe { std::slice::from_raw_parts_mut(buffer.cast::<u8>(), max_len as usize) };
        write_detect_string(buffer);
    });
}

/// Project metadata exported for diagnostics and packaging checks.
#[no_mangle]
pub extern "C" fn PdfWlxLicense() -> *const c_char {
    LICENSE.as_ptr().cast()
}

/// Project metadata exported for diagnostics and packaging checks.
#[no_mangle]
pub extern "C" fn PdfWlxVersion() -> *const c_char {
    VERSION.as_ptr().cast()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_page_count() {
        assert_eq!(page_count(b"Pages: 12\n"), Some(12));
        assert_eq!(page_count(b"Document\nPages: 0\n"), Some(0));
        assert_eq!(page_count(b"not a PDF"), None);
    }

    #[test]
    fn detection_string_is_terminated_and_truncated_safely() {
        let mut full = [0xff; 32];
        write_detect_string(&mut full);
        assert_eq!(&full[..DETECT_STRING.len()], DETECT_STRING);
        assert_eq!(full[DETECT_STRING.len()], 0);

        let mut short = [0xff; 4];
        write_detect_string(&mut short);
        assert_eq!(short, [b'E', b'X', b'T', 0]);
    }

    #[test]
    fn empty_detection_buffer_is_untouched() {
        let mut empty = [];
        write_detect_string(&mut empty);
    }
}
