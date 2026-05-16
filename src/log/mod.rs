//! High-level `Logger` + free-function logging API.

use core::ptr;
use std::ffi::CString;

use crate::error::LogError;
use crate::ffi;

/// Apple's log levels (mirrors `os_log_type_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Level {
    /// Normal informational message. Visible in Console.app by default.
    Default = ffi::level::DEFAULT,
    /// Informational. Visible when filtering Info-level on.
    Info = ffi::level::INFO,
    /// Debug detail. Hidden by default; visible with `log show --info --debug`.
    Debug = ffi::level::DEBUG,
    /// Error condition. Persists across log rotations.
    Error = ffi::level::ERROR,
    /// Catastrophic failure. Persisted + flagged.
    Fault = ffi::level::FAULT,
}

/// Wraps an `os_log_t` handle scoped to a subsystem + category.
///
/// `Logger` is `Send + Sync` — Apple's `os_log_t` is thread-safe.
pub struct Logger {
    raw: ffi::os_log_t,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Drop for Logger {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::apple_log_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Logger {
    /// Create a new logger for `subsystem` (e.g. `"fish.doom.myapp"`) and
    /// `category` (e.g. `"net"`).
    ///
    /// # Errors
    ///
    /// Returns [`LogError::InvalidArgument`] if either string contains a
    /// NUL byte, or [`LogError::CreateFailed`] if `os_log_create` returns
    /// NULL.
    pub fn new(subsystem: &str, category: &str) -> Result<Self, LogError> {
        let s =
            CString::new(subsystem).map_err(|e| LogError::InvalidArgument(e.to_string()))?;
        let c =
            CString::new(category).map_err(|e| LogError::InvalidArgument(e.to_string()))?;
        let raw = unsafe { ffi::apple_log_create(s.as_ptr(), c.as_ptr()) };
        if raw.is_null() {
            Err(LogError::CreateFailed)
        } else {
            Ok(Self { raw })
        }
    }

    /// Emit `message` at the requested level.
    pub fn log(&self, level: Level, message: &str) {
        let Ok(c) = CString::new(message.replace('\0', "\u{fffd}")) else {
            return;
        };
        unsafe { ffi::apple_log_emit(self.raw, level as i32, c.as_ptr()) };
    }

    /// Convenience: emit at `Level::Default`.
    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }
    /// Convenience: emit at `Level::Debug`.
    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }
    /// Convenience: emit at `Level::Error`.
    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }
    /// Convenience: emit at `Level::Fault`.
    pub fn fault(&self, message: &str) {
        self.log(Level::Fault, message);
    }
}

/// Emit `message` at the requested `level` via `OS_LOG_DEFAULT` (the
/// catch-all process logger — no subsystem / category).
///
/// Use a [`Logger`] when you want to filter via `log stream --predicate
/// 'subsystem == "fish.doom.myapp"'`.
pub fn log(level: Level, message: &str) {
    let Ok(c) = CString::new(message.replace('\0', "\u{fffd}")) else {
        return;
    };
    unsafe { ffi::apple_log_emit_default(level as i32, c.as_ptr()) };
}
