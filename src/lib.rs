// Copyright (C) 2026 Martin Brozkeff Malec
// Licensed under the EUPL, Version 1.2.

//! Production Qt5 WLX plugin for Double Commander.
//!
//! Rust owns WLX validation, rendering, temporary files, and panic
//! containment. A deliberately small C++ shim owns the QWidget operations
//! because Qt does not expose a stable C ABI.

use std::ffi::{c_char, c_void, CStr, CString, OsStr};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_PAGES: usize = 128;
const RENDER_DPI: u32 = 120;
const DETECT_STRING: &[u8] = b"EXT=\"PDF\"";
const LICENSE: &[u8] = b"Copyright (C) 2026 Martin Brozkeff Malec; licensed under the EUPL 1.2\0";
const VERSION: &[u8] = b"0.2.0-rust-qt5\0";
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

unsafe extern "C" {
    fn pdf_wlx_qt5_create(
        parent_handle: *mut c_void,
        page_paths: *const *const c_char,
        page_count: usize,
    ) -> *mut c_void;
    fn pdf_wlx_qt5_destroy(window_handle: *mut c_void);
}

#[derive(Debug)]
struct TempPages {
    directory: PathBuf,
    pages: Vec<PathBuf>,
}

impl Drop for TempPages {
    fn drop(&mut self) {
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
            "doublecmd-pdf-rust-qt5-{}-{timestamp}-{nonce}",
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
    let status = Command::new("mutool")
        .args(["draw", "-q", "-r"])
        .arg(RENDER_DPI.to_string())
        .arg("-o")
        .arg(directory.join("%d.png"))
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

fn path_from_c_string(file_to_load: *const c_char) -> PathBuf {
    // SAFETY: WLX supplies a valid NUL-terminated filename for the duration
    // of ListLoad, which validated that the pointer is non-null.
    let bytes = unsafe { CStr::from_ptr(file_to_load).to_bytes() };
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        PathBuf::from(OsStr::from_bytes(bytes))
    }
    #[cfg(not(unix))]
    {
        PathBuf::from(String::from_utf8_lossy(bytes).into_owned())
    }
}

fn load_plugin(parent: *mut c_void, file_to_load: *const c_char) -> *mut c_void {
    if parent.is_null() || file_to_load.is_null() {
        return std::ptr::null_mut();
    }
    let rendered = match render_pages(&path_from_c_string(file_to_load)) {
        Ok(rendered) => rendered,
        Err(()) => return std::ptr::null_mut(),
    };
    let paths: Result<Vec<_>, _> = rendered
        .pages
        .iter()
        .map(|path| CString::new(path.as_os_str().as_encoded_bytes()))
        .collect();
    let Ok(paths) = paths else {
        return std::ptr::null_mut();
    };
    let path_pointers: Vec<_> = paths.iter().map(|path| path.as_ptr()).collect();

    // SAFETY: Double Commander calls ListLoad on its Qt GUI thread and gives
    // us a live QWidget parent. All path pointers remain valid for this call;
    // the C++ shim loads each QPixmap synchronously and retains no pointer.
    unsafe { pdf_wlx_qt5_create(parent, path_pointers.as_ptr(), path_pointers.len()) }
}

fn guarded<T>(operation: impl FnOnce() -> T) -> Option<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)).ok()
}

#[no_mangle]
pub extern "C" fn ListLoad(
    parent: *mut c_void,
    file_to_load: *const c_char,
    _show_flags: i32,
) -> *mut c_void {
    guarded(|| load_plugin(parent, file_to_load)).unwrap_or(std::ptr::null_mut())
}

/// Closes the QWidget previously returned by [`ListLoad`].
///
/// # Safety
///
/// `list_window` must be null or the live handle returned by this plugin, and
/// the function must be called on the Qt GUI thread exactly once.
#[no_mangle]
pub unsafe extern "C" fn ListCloseWindow(list_window: *mut c_void) {
    let _ = guarded(|| {
        if !list_window.is_null() {
            // SAFETY: WLX returns the same live QWidget handle created by our
            // shim, and the host invokes close on its Qt GUI thread.
            unsafe { pdf_wlx_qt5_destroy(list_window) };
        }
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

#[no_mangle]
pub extern "C" fn ListGetDetectString(buffer: *mut c_char, max_len: i32) {
    let _ = guarded(|| {
        if buffer.is_null() || max_len <= 0 {
            return;
        }
        // SAFETY: WLX supplies a writable buffer containing max_len bytes.
        let output =
            unsafe { std::slice::from_raw_parts_mut(buffer.cast::<u8>(), max_len as usize) };
        write_detect_string(output);
    });
}

#[no_mangle]
pub extern "C" fn PdfWlxLicense() -> *const c_char {
    LICENSE.as_ptr().cast()
}

#[no_mangle]
pub extern "C" fn PdfWlxVersion() -> *const c_char {
    VERSION.as_ptr().cast()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_page_count() {
        assert_eq!(page_count(b"PDF-1.7\nPages: 3\n"), Some(3));
        assert_eq!(page_count(b"invalid"), None);
    }

    #[test]
    fn writes_terminated_detection_string() {
        let mut output = [0xff; 4];
        write_detect_string(&mut output);
        assert_eq!(output, [b'E', b'X', b'T', 0]);
    }
}
