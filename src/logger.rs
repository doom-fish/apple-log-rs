#![allow(clippy::missing_panics_doc, clippy::should_implement_trait)]

use core::ffi::c_void;
use std::ffi::CStr;
use std::ptr::NonNull;

use crate::bridge_support::{bridge_ptr_result, c_string_arg, sanitized_c_string};
use crate::error::LogError;
use crate::ffi;
use crate::ffi::signpost_kind;
use crate::os_log::{Level, OSLog};
use crate::os_signpost_id::OSSignpostId;

/// Controls whether logged string payloads are persisted in clear text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Privacy {
    Public,
    Private,
}

impl Privacy {
    const fn raw(self) -> i32 {
        match self {
            Self::Public => 0,
            Self::Private => 1,
        }
    }
}

const fn severity_for_level(level: Level) -> i32 {
    match level {
        Level::Default => 0,
        Level::Debug => 2,
        Level::Info => 3,
        Level::Error => 6,
        Level::Fault => 8,
    }
}

/// Swift `Logger` backed by an `OSLog` handle.
pub struct Logger {
    ptr: NonNull<c_void>,
}

impl Logger {
    fn bridge_default() -> Self {
        Self {
            ptr: NonNull::new(unsafe {
                // SAFETY: ffi::apple_log_logger_default is a thin wrapper that returns
                // a non-null OSLog/Logger handle from the Swift bridge. It is safe to call
                // and always returns a valid pointer.
                ffi::apple_log_logger_default()
            })
            .expect("Swift bridge never returns NULL for Logger.default"),
        }
    }

    /// Creates a logger for a subsystem/category pair.
    ///
    /// # Errors
    ///
    /// Returns an error if either argument contains a NUL byte or the bridge fails.
    pub fn new(subsystem: &str, category: &str) -> Result<Self, LogError> {
        let subsystem = c_string_arg("subsystem", subsystem)?;
        let category = c_string_arg("category", category)?;
        let ptr = bridge_ptr_result("Logger::new", |error_out| unsafe {
            // SAFETY: c_string_arg validates both arguments are valid C strings.
            // The Swift bridge ffi::apple_log_logger_create is thread-safe and takes
            // valid C pointers plus an error_out pointer populated by bridge_ptr_result.
            ffi::apple_log_logger_create(subsystem.as_ptr(), category.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Creates a logger from an existing `OSLog` handle.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge fails to create the logger wrapper.
    pub fn from_os_log(log: &OSLog) -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("Logger::from_os_log", |error_out| unsafe {
            // SAFETY: log.as_ptr() returns a valid OSLog pointer from the OSLog wrapper.
            // The Swift bridge is thread-safe and error_out is populated by bridge_ptr_result.
            ffi::apple_log_logger_from_os_log(log.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Returns `Logger()`.
    #[must_use]
    pub fn default() -> Self {
        Self::bridge_default()
    }

    /// Returns a disabled logger.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            ptr: NonNull::new(unsafe {
                // SAFETY: ffi::apple_log_logger_disabled is a thin wrapper that returns
                // a non-null Logger handle from the Swift bridge. It is safe to call
                // and always returns a valid pointer.
                ffi::apple_log_logger_disabled()
            })
            .expect("Swift bridge never returns NULL for Logger.disabled"),
        }
    }

    fn write(&self, severity: i32, privacy: Privacy, message: &str) {
        let message = sanitized_c_string(message);
        unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer from the Swift bridge.
            // sanitized_c_string validates the message is a valid C string.
            // The function is thread-safe and does not mutate through self.
            ffi::apple_log_logger_log(self.ptr.as_ptr(), severity, privacy.raw(), message.as_ptr());
        }
    }

    /// Emits a message at one of the classic `OSLogType` levels.
    pub fn log(&self, level: Level, message: &str) {
        self.write(severity_for_level(level), Privacy::Private, message);
    }

    /// Emits a message with an explicit privacy level.
    pub fn log_with_privacy(&self, level: Level, message: &str, privacy: Privacy) {
        self.write(severity_for_level(level), privacy, message);
    }

    pub fn trace(&self, message: &str) {
        self.write(1, Privacy::Private, message);
    }

    pub fn debug(&self, message: &str) {
        self.write(2, Privacy::Private, message);
    }

    pub fn info(&self, message: &str) {
        self.write(3, Privacy::Private, message);
    }

    pub fn notice(&self, message: &str) {
        self.write(4, Privacy::Private, message);
    }

    pub fn warning(&self, message: &str) {
        self.write(5, Privacy::Private, message);
    }

    pub fn error(&self, message: &str) {
        self.write(6, Privacy::Private, message);
    }

    pub fn critical(&self, message: &str) {
        self.write(7, Privacy::Private, message);
    }

    pub fn fault(&self, message: &str) {
        self.write(8, Privacy::Private, message);
    }

    /// Returns whether the underlying `OSLog` handle enables this classic level.
    #[must_use]
    pub fn is_enabled(&self, level: Level) -> bool {
        unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer. The FFI function
            // is a read-only query and safe to call from any thread.
            ffi::apple_log_logger_is_enabled(self.ptr.as_ptr(), level as u8)
        }
    }

    /// Generates a signpost identifier associated with this logger's log handle.
    #[must_use]
    pub fn signpost_id(&self) -> OSSignpostId {
        OSSignpostId::from_u64(unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer.
            // This is a read-only operation that generates a signpost ID.
            ffi::apple_log_logger_signpost_id(self.ptr.as_ptr())
        })
    }

    /// Generates a signpost identifier derived from a pointer.
    #[must_use]
    pub fn signpost_id_from_pointer<T>(&self, pointer: *const T) -> OSSignpostId {
        OSSignpostId::from_u64(unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer. The pointer argument
            // is cast to c_void which is safe for any pointer type. The function only
            // reads from the pointer value (not dereferences it) to generate a signpost ID.
            ffi::apple_log_logger_signpost_id_from_pointer(self.ptr.as_ptr(), pointer.cast())
        })
    }

    /// Returns whether signposts are enabled for this logger's log handle.
    #[must_use]
    pub fn signposts_enabled(&self) -> bool {
        unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer.
            // This is a read-only query that is safe to call from any thread.
            ffi::apple_log_logger_signposts_enabled(self.ptr.as_ptr())
        }
    }

    fn signpost_emit(
        &self,
        kind: i32,
        id: OSSignpostId,
        name: &'static CStr,
        message: &str,
        privacy: Privacy,
    ) {
        let message = (!message.is_empty()).then(|| sanitized_c_string(message));
        unsafe {
            ffi::apple_log_logger_signpost_emit(
                self.ptr.as_ptr(),
                id.as_u64(),
                kind,
                name.as_ptr(),
                message
                    .as_ref()
                    .map_or(std::ptr::null(), |message| message.as_ptr()),
                privacy == Privacy::Public,
            );
        }
    }

    pub fn signpost_event(&self, id: OSSignpostId, name: &'static CStr, message: &str) {
        self.signpost_emit(signpost_kind::EVENT, id, name, message, Privacy::Private);
    }

    pub fn signpost_event_with_privacy(
        &self,
        id: OSSignpostId,
        name: &'static CStr,
        message: &str,
        privacy: Privacy,
    ) {
        self.signpost_emit(signpost_kind::EVENT, id, name, message, privacy);
    }

    pub fn signpost_interval_begin(&self, id: OSSignpostId, name: &'static CStr) {
        self.signpost_emit(
            signpost_kind::INTERVAL_BEGIN,
            id,
            name,
            "",
            Privacy::Private,
        );
    }

    pub fn signpost_animation_interval_begin(&self, id: OSSignpostId, name: &'static CStr) {
        self.signpost_emit(
            signpost_kind::ANIMATION_INTERVAL_BEGIN,
            id,
            name,
            "",
            Privacy::Private,
        );
    }

    pub fn signpost_interval_end(&self, id: OSSignpostId, name: &'static CStr) {
        self.signpost_emit(signpost_kind::INTERVAL_END, id, name, "", Privacy::Private);
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::bridge_default()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: self.ptr is a valid non-null Logger pointer that was created by
            // the Swift bridge. apple_log_logger_release is safe to call exactly once
            // per Logger and properly cleans up the Swift bridge reference count.
            ffi::apple_log_logger_release(self.ptr.as_ptr());
        }
    }
}
