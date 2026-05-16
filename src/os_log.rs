#![allow(clippy::missing_panics_doc, clippy::should_implement_trait)]

use core::ffi::c_void;
use std::ptr::NonNull;

use crate::bridge_support::{bridge_ptr_result, c_string_arg, take_optional_c_string};
use crate::error::LogError;
use crate::ffi;

pub const CATEGORY_POINTS_OF_INTEREST: &str = ffi::category::POINTS_OF_INTEREST;
pub const CATEGORY_DYNAMIC_TRACING: &str = ffi::category::DYNAMIC_TRACING;
pub const CATEGORY_DYNAMIC_STACK_TRACING: &str = ffi::category::DYNAMIC_STACK_TRACING;

/// Apple's log levels (`OSLogType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Level {
    Default = ffi::level::DEFAULT,
    Info = ffi::level::INFO,
    Debug = ffi::level::DEBUG,
    Error = ffi::level::ERROR,
    Fault = ffi::level::FAULT,
}

/// Safe wrapper around `OSLog` / `os_log_t` handles.
pub struct OSLog {
    ptr: NonNull<c_void>,
}

impl OSLog {
    fn bridge_default() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_log_default() })
                .expect("Swift bridge never returns NULL for OSLog.default"),
        }
    }

    /// Creates a log handle for a subsystem/category pair.
    ///
    /// # Errors
    ///
    /// Returns an error if either string contains a NUL byte or the bridge fails.
    pub fn new(subsystem: &str, category: &str) -> Result<Self, LogError> {
        let subsystem = c_string_arg("subsystem", subsystem)?;
        let category = c_string_arg("category", category)?;
        let ptr = bridge_ptr_result("OSLog::new", |error_out| unsafe {
            ffi::apple_log_os_log_create(subsystem.as_ptr(), category.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Returns `OSLog.default`.
    #[must_use]
    pub fn default() -> Self {
        Self::bridge_default()
    }

    /// Returns `OSLog.disabled`.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_log_disabled() })
                .expect("Swift bridge never returns NULL for OSLog.disabled"),
        }
    }

    /// Returns whether a log level is enabled.
    #[must_use]
    pub fn is_enabled(&self, level: Level) -> bool {
        unsafe { ffi::apple_log_os_log_is_enabled(self.ptr.as_ptr(), level as u8) }
    }

    /// Returns whether signposts are enabled for this log handle.
    #[must_use]
    pub fn signposts_enabled(&self) -> bool {
        unsafe { ffi::apple_log_os_log_signposts_enabled(self.ptr.as_ptr()) }
    }

    /// Returns the subsystem used to create this log handle, if the bridge knows it.
    #[must_use]
    pub fn subsystem(&self) -> Option<String> {
        unsafe { take_optional_c_string(ffi::apple_log_os_log_copy_subsystem(self.ptr.as_ptr())) }
    }

    /// Returns the category used to create this log handle, if the bridge knows it.
    #[must_use]
    pub fn category(&self) -> Option<String> {
        unsafe { take_optional_c_string(ffi::apple_log_os_log_copy_category(self.ptr.as_ptr())) }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl Default for OSLog {
    fn default() -> Self {
        Self::bridge_default()
    }
}

impl Drop for OSLog {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_release(self.ptr.as_ptr()) };
    }
}
