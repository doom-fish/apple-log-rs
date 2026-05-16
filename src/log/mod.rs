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

    /// True if Apple's log subsystem would actually record a message
    /// at `level` for this logger right now. Use to skip expensive
    /// formatting when nobody is listening. Wraps `os_log_type_enabled`.
    #[must_use]
    pub fn is_enabled(&self, level: Level) -> bool {
        unsafe { ffi::apple_log_type_enabled(self.raw, level as i32) }
    }

    /// Generate a fresh signpost id scoped to this logger. Pair with
    /// [`Self::signpost_event`] / [`Self::signpost_interval_begin`].
    /// Wraps `os_signpost_id_generate`.
    #[must_use]
    pub fn signpost_id(&self) -> SignpostId {
        SignpostId(unsafe { ffi::apple_signpost_id_generate(self.raw) })
    }

    /// True if signposts are currently being recorded for this logger
    /// (e.g. Instruments is attached). Wraps `os_signpost_enabled`.
    #[must_use]
    pub fn signposts_enabled(&self) -> bool {
        unsafe { ffi::apple_signpost_enabled(self.raw) }
    }

    /// Emit an instantaneous performance event. Visible in Instruments'
    /// "Points of Interest" track. Wraps `os_signpost_event_emit`.
    pub fn signpost_event(&self, id: SignpostId, name: &str, message: &str) {
        let Ok(n) = CString::new(name.replace('\0', "\u{fffd}")) else {
            return;
        };
        let Ok(m) = CString::new(message.replace('\0', "\u{fffd}")) else {
            return;
        };
        unsafe { ffi::apple_signpost_event_emit(self.raw, id.0, n.as_ptr(), m.as_ptr()) };
    }

    /// Begin a timed interval. Pair with [`Self::signpost_interval_end`]
    /// using the same `id` and `name`. Wraps `os_signpost_interval_begin`.
    pub fn signpost_interval_begin(&self, id: SignpostId, name: &str) {
        let Ok(n) = CString::new(name.replace('\0', "\u{fffd}")) else {
            return;
        };
        unsafe { ffi::apple_signpost_interval_begin(self.raw, id.0, n.as_ptr()) };
    }

    /// End a timed interval started with [`Self::signpost_interval_begin`].
    /// Wraps `os_signpost_interval_end`.
    pub fn signpost_interval_end(&self, id: SignpostId, name: &str) {
        let Ok(n) = CString::new(name.replace('\0', "\u{fffd}")) else {
            return;
        };
        unsafe { ffi::apple_signpost_interval_end(self.raw, id.0, n.as_ptr()) };
    }
}

/// Opaque signpost identifier — pair `_begin` and `_end` calls with
/// the same id to mark a single timed region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignpostId(u64);

impl SignpostId {
    /// Raw `os_signpost_id_t` value.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

/// Return the id of the currently-active `os_activity` (0 if none).
/// Useful for correlating log messages within an activity scope.
#[must_use]
pub fn active_activity_id() -> u64 {
    unsafe { ffi::apple_activity_get_active_id() }
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
