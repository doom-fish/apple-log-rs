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
    match time.duration_since(UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_secs_f64(),
        Err(error) => -error.duration().as_secs_f64(),
    }
}

pub fn secs_to_system_time(seconds: f64) -> SystemTime {
    Duration::try_from_secs_f64(seconds.abs())
        .ok()
        .and_then(|offset| {
            if seconds.is_sign_negative() {
                UNIX_EPOCH.checked_sub(offset)
            } else {
                UNIX_EPOCH.checked_add(offset)
            }
        })
        .unwrap_or(UNIX_EPOCH)
}

pub trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secs_to_system_time_never_panics_on_invalid_dates() {
        for seconds in [
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            f64::MAX,
            f64::MIN,
            1e300,
        ] {
            assert_eq!(secs_to_system_time(seconds), UNIX_EPOCH, "{seconds}");
        }
    }

    #[test]
    fn secs_to_system_time_handles_dates_on_both_sides_of_the_epoch() {
        assert_eq!(
            secs_to_system_time(1.5),
            UNIX_EPOCH + Duration::from_millis(1500)
        );
        assert_eq!(
            secs_to_system_time(-2.0),
            UNIX_EPOCH - Duration::from_secs(2)
        );
        assert_eq!(secs_to_system_time(-0.0), UNIX_EPOCH);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn system_time_to_secs_is_signed() {
        let before = UNIX_EPOCH - Duration::from_secs(10);
        let after = UNIX_EPOCH + Duration::from_secs(10);
        assert_eq!(system_time_to_secs(before), -10.0);
        assert_eq!(system_time_to_secs(after), 10.0);
        assert_eq!(secs_to_system_time(system_time_to_secs(before)), before);
        assert_eq!(secs_to_system_time(system_time_to_secs(after)), after);
    }
}
