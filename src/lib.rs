// Copyright (C) 2026 Martin Brozkeff Malec
// SPDX-License-Identifier: EUPL-1.2 OR GPL-2.0-or-later OR AGPL-3.0-or-later

//! Production Qt5 WLX plugin for Double Commander.
//!
//! Rust owns the WLX entry points, panic containment, and the `mutool`
//! subprocess backend. The C++ shim owns Qt widgets and Poppler's C++ API calls
//! because neither library exposes a stable C ABI.

use std::ffi::{c_char, c_void};
#[cfg(feature = "mutool")]
use std::ffi::{CStr, CString, OsStr};
#[cfg(feature = "mutool")]
use std::fs;
#[cfg(feature = "mutool")]
use std::path::{Path, PathBuf};
#[cfg(feature = "mutool")]
use std::process::Command;
#[cfg(feature = "mutool")]
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "mutool")]
use std::time::{SystemTime, UNIX_EPOCH};

const RENDER_DPI: u32 = 120;
const DETECT_STRING: &[u8] = b"EXT=\"PDF\"";
#[cfg(feature = "poppler-splash")]
const LICENSE: &[u8] =
    b"GPL-2.0-or-later; project source also EUPL-1.2; see THIRD-PARTY-NOTICES.md\0";
#[cfg(feature = "mutool")]
const LICENSE: &[u8] =
    b"AGPL-3.0-or-later; project source also EUPL-1.2; see THIRD-PARTY-NOTICES.md\0";
#[cfg(feature = "poppler-splash")]
const VERSION: &[u8] = b"0.3.0-poppler-splash-qt5\0";
#[cfg(feature = "mutool")]
const VERSION: &[u8] = b"0.3.0-mutool-qt5\0";
#[cfg(feature = "mutool")]
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
#[path = "../tests/unit/plugin.rs"]
mod tests;

unsafe extern "C" {
    #[cfg(feature = "mutool")]
    fn pdf_wlx_qt5_create(
        parent_handle: *mut c_void,
        page_paths: *const *const c_char,
        page_count: usize,
        document_page_count: usize,
    ) -> *mut c_void;
    #[cfg(feature = "poppler-splash")]
    fn pdf_wlx_poppler_create(
        parent_handle: *mut c_void,
        file_name: *const c_char,
        render_dpi: u32,
    ) -> *mut c_void;
    fn pdf_wlx_qt5_destroy(window_handle: *mut c_void);
}

#[cfg(feature = "mutool")]
#[derive(Debug)]
struct TempPages {
    directory: PathBuf,
    pages: Vec<PathBuf>,
    document_pages: usize,
}

#[cfg(feature = "mutool")]
impl Drop for TempPages {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

#[cfg(feature = "mutool")]
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
                if fs::set_permissions(&directory, fs::Permissions::from_mode(0o700)).is_err() {
                    let _ = fs::remove_dir(&directory);
                    return Err(());
                }
            }
            return Ok(directory);
        }
    }
    Err(())
}

#[cfg(feature = "mutool")]
fn page_count(output: &[u8]) -> Option<usize> {
    std::str::from_utf8(output)
        .ok()?
        .lines()
        .find_map(|line| line.strip_prefix("Pages:")?.trim().parse().ok())
}

#[cfg(feature = "mutool")]
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
    if pages == 0 {
        return Err(());
    }

    let directory = temporary_directory()?;
    let mut rendered = TempPages {
        directory,
        pages: Vec::with_capacity(1),
        document_pages: pages,
    };
    let status = Command::new("mutool")
        .args(["draw", "-q", "-r"])
        .arg(RENDER_DPI.to_string())
        .arg("-o")
        .arg(rendered.directory.join("%d.png"))
        .arg(file_name)
        .arg("1")
        .status()
        .map_err(|_| ())?;
    if !status.success() {
        return Err(());
    }

    let first_page = rendered.directory.join("1.png");
    if !first_page.is_file() {
        return Err(());
    }
    rendered.pages.push(first_page);
    Ok(rendered)
}

#[cfg(feature = "mutool")]
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

#[cfg(feature = "mutool")]
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
    unsafe {
        pdf_wlx_qt5_create(
            parent,
            path_pointers.as_ptr(),
            path_pointers.len(),
            rendered.document_pages,
        )
    }
}

#[cfg(feature = "poppler-splash")]
fn load_plugin(parent: *mut c_void, file_to_load: *const c_char) -> *mut c_void {
    if parent.is_null() || file_to_load.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: WLX supplies a valid filename pointer for this call and the
    // Qt5 shim copies the rendered page into a QWidget before returning.
    unsafe { pdf_wlx_poppler_create(parent, file_to_load, RENDER_DPI) }
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
