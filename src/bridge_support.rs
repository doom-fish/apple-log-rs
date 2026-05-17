use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::LogError;
use crate::ffi;

pub type ErrorOut = *mut *mut c_char;

pub fn bridge_result<T>(f: impl FnOnce(ErrorOut) -> T) -> Result<T, LogError> {
    let mut error = ptr::null_mut();
    let value = f(&mut error);
    if error.is_null() {
        Ok(value)
    } else {
        Err(LogError::bridge(unsafe { take_owned_c_string(error) }))
    }
}

pub fn bridge_ptr_result(
    label: &'static str,
    f: impl FnOnce(ErrorOut) -> *mut c_void,
) -> Result<std::ptr::NonNull<c_void>, LogError> {
    bridge_result(f)?.pipe(|ptr| {
        std::ptr::NonNull::new(ptr)
            .ok_or_else(|| LogError::bridge(format!("{label} returned NULL")))
    })
}

pub fn c_string_arg(label: &str, value: &str) -> Result<CString, LogError> {
    CString::new(value)
        .map_err(|_| LogError::InvalidArgument(format!("{label} contained a NUL byte")))
}

pub fn sanitized_c_string(value: &str) -> CString {
    CString::new(value.replace('\0', "\u{fffd}")).expect("replacement string never contains NUL")
}

pub fn path_c_string(path: &Path) -> Result<CString, LogError> {
    c_string_arg("path", &path.to_string_lossy())
}

pub unsafe fn take_owned_c_string(ptr: *mut c_char) -> String {
    let result = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::apple_log_string_free(ptr);
    result
}

pub unsafe fn take_optional_c_string(ptr: *mut c_char) -> Option<String> {
    (!ptr.is_null()).then(|| take_owned_c_string(ptr))
}

pub unsafe fn take_owned_bytes(ptr: *mut c_void, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        ffi::apple_log_bytes_free(ptr);
        return Vec::new();
    }
    let slice = std::slice::from_raw_parts(ptr.cast::<u8>(), len);
    let result = slice.to_vec();
    ffi::apple_log_bytes_free(ptr);
    result
}

pub fn system_time_to_secs(time: SystemTime) -> f64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs_f64()
}

pub fn secs_to_system_time(seconds: f64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs_f64(seconds.max(0.0))
}

pub trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}
