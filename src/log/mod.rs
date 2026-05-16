//! High-level `Logger` + free-function logging API.

use core::ptr;
use std::ffi::CString;

use crate::error::LogError;
use crate::ffi;

pub const CATEGORY_POINTS_OF_INTEREST: &str = ffi::category::POINTS_OF_INTEREST;
pub const CATEGORY_DYNAMIC_TRACING: &str = ffi::category::DYNAMIC_TRACING;
pub const CATEGORY_DYNAMIC_STACK_TRACING: &str = ffi::category::DYNAMIC_STACK_TRACING;

/// Apple's log levels (mirrors `os_log_type_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Level {
    Default = ffi::level::DEFAULT,
    Info = ffi::level::INFO,
    Debug = ffi::level::DEBUG,
    Error = ffi::level::ERROR,
    Fault = ffi::level::FAULT,
}

/// Controls whether string payloads are persisted in clear text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Privacy {
    Public,
    Private,
}

impl Privacy {
    #[must_use]
    const fn is_public(self) -> bool {
        matches!(self, Self::Public)
    }
}

/// Wraps an `os_log_t` handle scoped to a subsystem + category.
///
/// `Logger` is `Send + Sync` — Apple's `os_log_t` is thread-safe.
pub struct Logger {
    raw: ffi::os_log_t,
    owned: bool,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Default for Logger {
    fn default() -> Self {
        Self {
            raw: unsafe { ffi::apple_log_default() },
            owned: false,
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        if self.owned && !self.raw.is_null() {
            unsafe { ffi::apple_log_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Logger {
    /// Create a new logger for `subsystem` and `category`.
    ///
    /// # Errors
    ///
    /// Returns [`LogError::InvalidArgument`] if either string contains a
    /// NUL byte, or [`LogError::CreateFailed`] if `os_log_create` returns
    /// NULL.
    pub fn new(subsystem: &str, category: &str) -> Result<Self, LogError> {
        let subsystem =
            CString::new(subsystem).map_err(|err| LogError::InvalidArgument(err.to_string()))?;
        let category =
            CString::new(category).map_err(|err| LogError::InvalidArgument(err.to_string()))?;
        let raw = unsafe { ffi::apple_log_create(subsystem.as_ptr(), category.as_ptr()) };
        if raw.is_null() {
            Err(LogError::CreateFailed)
        } else {
            Ok(Self { raw, owned: true })
        }
    }

    /// Return a logger backed by `OS_LOG_DISABLED`.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            raw: unsafe { ffi::apple_log_disabled() },
            owned: false,
        }
    }

    /// Emit `message` at the requested level with public visibility.
    pub fn log(&self, level: Level, message: &str) {
        self.log_with_privacy(level, message, Privacy::Public);
    }

    /// Emit `message` at the requested level with explicit privacy.
    pub fn log_with_privacy(&self, level: Level, message: &str, privacy: Privacy) {
        let Ok(message) = sanitize_message(message) else {
            return;
        };
        unsafe {
            ffi::apple_log_emit_privacy(
                self.raw,
                level as i32,
                message.as_ptr(),
                privacy.is_public(),
            );
        };
    }

    pub fn info(&self, message: &str) {
        self.log(Level::Info, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(Level::Debug, message);
    }

    pub fn error(&self, message: &str) {
        self.log(Level::Error, message);
    }

    pub fn fault(&self, message: &str) {
        self.log(Level::Fault, message);
    }

    #[must_use]
    pub fn is_enabled(&self, level: Level) -> bool {
        unsafe { ffi::apple_log_type_enabled(self.raw, level as i32) }
    }

    #[must_use]
    pub fn signpost_id(&self) -> SignpostId {
        SignpostId(unsafe { ffi::apple_signpost_id_generate(self.raw) })
    }

    #[must_use]
    pub fn signpost_id_from_pointer<T>(&self, pointer: *const T) -> SignpostId {
        SignpostId(unsafe { ffi::apple_signpost_id_make_with_pointer(self.raw, pointer.cast()) })
    }

    #[must_use]
    pub fn signposts_enabled(&self) -> bool {
        unsafe { ffi::apple_signpost_enabled(self.raw) }
    }

    pub fn signpost_event(&self, id: SignpostId, name: &str, message: &str) {
        let Ok(name) = sanitize_message(name) else {
            return;
        };
        let Ok(message) = sanitize_message(message) else {
            return;
        };
        unsafe { ffi::apple_signpost_event_emit(self.raw, id.0, name.as_ptr(), message.as_ptr()) };
    }

    pub fn signpost_interval_begin(&self, id: SignpostId, name: &str) {
        let Ok(name) = sanitize_message(name) else {
            return;
        };
        unsafe { ffi::apple_signpost_interval_begin(self.raw, id.0, name.as_ptr()) };
    }

    pub fn signpost_animation_interval_begin(&self, id: SignpostId, name: &str) {
        let Ok(name) = sanitize_message(name) else {
            return;
        };
        unsafe { ffi::apple_signpost_animation_interval_begin(self.raw, id.0, name.as_ptr()) };
    }

    pub fn signpost_interval_end(&self, id: SignpostId, name: &str) {
        let Ok(name) = sanitize_message(name) else {
            return;
        };
        unsafe { ffi::apple_signpost_interval_end(self.raw, id.0, name.as_ptr()) };
    }
}

/// Opaque signpost identifier — pair `_begin` and `_end` calls with
/// the same id to mark a single timed region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignpostId(u64);

impl SignpostId {
    pub const NULL: Self = Self(ffi::signpost_id::NULL);
    pub const INVALID: Self = Self(ffi::signpost_id::INVALID);
    pub const EXCLUSIVE: Self = Self(ffi::signpost_id::EXCLUSIVE);

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_u64(raw: u64) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0 == Self::NULL.0
    }

    #[must_use]
    pub const fn is_invalid(self) -> bool {
        self.0 == Self::INVALID.0
    }
}

/// Current activity id plus its optional parent id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivityIds {
    pub current: u64,
    pub parent: Option<u64>,
}

/// Return the id of the currently-active `os_activity` (0 if none).
#[must_use]
pub fn active_activity_id() -> u64 {
    active_activity_ids().current
}

/// Return the current `os_activity` id and its parent id, if any.
#[must_use]
pub fn active_activity_ids() -> ActivityIds {
    let mut parent = 0_u64;
    let current = unsafe { ffi::apple_activity_get_identifiers(&mut parent) };
    ActivityIds {
        current,
        parent: (parent != 0).then_some(parent),
    }
}

/// Return whether `OS_LOG_DEFAULT` would emit messages at `level` right now.
#[must_use]
pub fn log_enabled(level: Level) -> bool {
    unsafe { ffi::apple_log_type_enabled(ptr::null_mut(), level as i32) }
}

/// Emit `message` at the requested `level` via `OS_LOG_DEFAULT`.
pub fn log(level: Level, message: &str) {
    log_with_privacy(level, message, Privacy::Public);
}

/// Emit `message` at `level` via `OS_LOG_DEFAULT` with explicit privacy.
pub fn log_with_privacy(level: Level, message: &str, privacy: Privacy) {
    let Ok(message) = sanitize_message(message) else {
        return;
    };
    unsafe {
        ffi::apple_log_emit_default_privacy(level as i32, message.as_ptr(), privacy.is_public());
    };
}

fn sanitize_message(message: &str) -> Result<CString, std::ffi::NulError> {
    CString::new(message.replace('\0', "\u{fffd}"))
}
