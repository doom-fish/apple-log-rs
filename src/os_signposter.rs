#![allow(clippy::missing_panics_doc, clippy::should_implement_trait)]

use core::ffi::c_void;
use std::ptr::NonNull;

use crate::bridge_support::{bridge_ptr_result, c_string_arg, sanitized_c_string};
use crate::error::LogError;
use crate::ffi;
use crate::logger::Logger;
use crate::os_log::OSLog;
use crate::os_signpost_id::OSSignpostId;

/// A started signpost interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OSSignpostInterval {
    id: OSSignpostId,
    animation: bool,
}

impl OSSignpostInterval {
    #[must_use]
    pub const fn id(self) -> OSSignpostId {
        self.id
    }

    #[must_use]
    pub const fn is_animation(self) -> bool {
        self.animation
    }
}

/// Signpost emitter backed by an `OSLog` handle.
pub struct OSSignposter {
    ptr: NonNull<c_void>,
}

impl OSSignposter {
    fn bridge_default() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_signposter_default() })
                .expect("Swift bridge never returns NULL for OSSignposter.default"),
        }
    }

    /// Creates a signposter for a subsystem/category pair.
    ///
    /// # Errors
    ///
    /// Returns an error if either string contains a NUL byte or the bridge fails.
    pub fn new(subsystem: &str, category: &str) -> Result<Self, LogError> {
        let subsystem = c_string_arg("subsystem", subsystem)?;
        let category = c_string_arg("category", category)?;
        let ptr = bridge_ptr_result("OSSignposter::new", |error_out| unsafe {
            ffi::apple_log_os_signposter_create(subsystem.as_ptr(), category.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Creates a signposter from an `OSLog` handle.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge fails.
    pub fn from_os_log(log: &OSLog) -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSSignposter::from_os_log", |error_out| unsafe {
            ffi::apple_log_os_signposter_from_os_log(log.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Creates a signposter from a `Logger`.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge fails.
    pub fn from_logger(logger: &Logger) -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSSignposter::from_logger", |error_out| unsafe {
            ffi::apple_log_os_signposter_from_logger(logger.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    #[must_use]
    pub fn default() -> Self {
        Self::bridge_default()
    }

    #[must_use]
    pub fn disabled() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_signposter_disabled() })
                .expect("Swift bridge never returns NULL for OSSignposter.disabled"),
        }
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::apple_log_os_signposter_is_enabled(self.ptr.as_ptr()) }
    }

    #[must_use]
    pub fn make_signpost_id(&self) -> OSSignpostId {
        OSSignpostId::from_u64(unsafe { ffi::apple_log_os_signposter_make_signpost_id(self.ptr.as_ptr()) })
    }

    #[must_use]
    pub fn make_signpost_id_from_pointer<T>(&self, pointer: *const T) -> OSSignpostId {
        OSSignpostId::from_u64(unsafe {
            ffi::apple_log_os_signposter_make_signpost_id_from_pointer(self.ptr.as_ptr(), pointer.cast())
        })
    }

    pub fn emit_event(&self, name: &str, id: OSSignpostId, message: &str) {
        let name = sanitized_c_string(name);
        let message = sanitized_c_string(message);
        unsafe {
            ffi::apple_log_os_signposter_emit_event(
                self.ptr.as_ptr(),
                id.as_u64(),
                name.as_ptr(),
                message.as_ptr(),
            );
        }
    }

    #[must_use]
    pub fn begin_interval(&self, name: &str, id: OSSignpostId, message: &str) -> OSSignpostInterval {
        let name = sanitized_c_string(name);
        let message = sanitized_c_string(message);
        unsafe {
            ffi::apple_log_os_signposter_begin_interval(
                self.ptr.as_ptr(),
                id.as_u64(),
                name.as_ptr(),
                message.as_ptr(),
            );
        }
        OSSignpostInterval { id, animation: false }
    }

    #[must_use]
    pub fn begin_animation_interval(
        &self,
        name: &str,
        id: OSSignpostId,
        message: &str,
    ) -> OSSignpostInterval {
        let name = sanitized_c_string(name);
        let message = sanitized_c_string(message);
        unsafe {
            ffi::apple_log_os_signposter_begin_animation_interval(
                self.ptr.as_ptr(),
                id.as_u64(),
                name.as_ptr(),
                message.as_ptr(),
            );
        }
        OSSignpostInterval { id, animation: true }
    }

    pub fn end_interval(&self, name: &str, interval: OSSignpostInterval, message: &str) {
        let name = sanitized_c_string(name);
        let message = sanitized_c_string(message);
        unsafe {
            ffi::apple_log_os_signposter_end_interval(
                self.ptr.as_ptr(),
                interval.id.as_u64(),
                name.as_ptr(),
                message.as_ptr(),
            );
        }
    }

    pub fn with_interval_signpost<T>(
        &self,
        name: &str,
        id: OSSignpostId,
        message: &str,
        around: impl FnOnce() -> T,
    ) -> T {
        let interval = self.begin_interval(name, id, message);
        let result = around();
        self.end_interval(name, interval, message);
        result
    }
}

impl Default for OSSignposter {
    fn default() -> Self {
        Self::bridge_default()
    }
}

impl Drop for OSSignposter {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_signposter_release(self.ptr.as_ptr()) };
    }
}
