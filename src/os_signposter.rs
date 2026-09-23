#![allow(clippy::missing_panics_doc, clippy::should_implement_trait)]

use core::ffi::c_void;
use std::ffi::CStr;
use std::ptr::NonNull;

use crate::bridge_support::{bridge_ptr_result, c_string_arg, sanitized_c_string};
use crate::error::LogError;
use crate::ffi;
use crate::ffi::signpost_kind;
use crate::logger::{Logger, Privacy};
use crate::os_log::OSLog;
use crate::os_signpost_id::OSSignpostId;

/// A started signpost interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OSSignpostInterval {
    id: OSSignpostId,
    name: &'static CStr,
    animation: bool,
}

impl OSSignpostInterval {
    #[must_use]
    pub const fn id(self) -> OSSignpostId {
        self.id
    }

    #[must_use]
    pub const fn name(self) -> &'static CStr {
        self.name
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
        OSSignpostId::from_u64(unsafe {
            ffi::apple_log_os_signposter_make_signpost_id(self.ptr.as_ptr())
        })
    }

    #[must_use]
    pub fn make_signpost_id_from_pointer<T>(&self, pointer: *const T) -> OSSignpostId {
        OSSignpostId::from_u64(unsafe {
            ffi::apple_log_os_signposter_make_signpost_id_from_pointer(
                self.ptr.as_ptr(),
                pointer.cast(),
            )
        })
    }

    fn emit(
        &self,
        kind: i32,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
        privacy: Privacy,
    ) {
        let message = (!message.is_empty()).then(|| sanitized_c_string(message));
        unsafe {
            ffi::apple_log_os_signposter_emit(
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

    pub fn emit_event(&self, name: &'static CStr, id: OSSignpostId, message: &str) {
        self.emit(signpost_kind::EVENT, name, id, message, Privacy::Private);
    }

    pub fn emit_event_with_privacy(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
        privacy: Privacy,
    ) {
        self.emit(signpost_kind::EVENT, name, id, message, privacy);
    }

    #[must_use]
    pub fn begin_interval(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
    ) -> OSSignpostInterval {
        self.begin_interval_with_privacy(name, id, message, Privacy::Private)
    }

    #[must_use]
    pub fn begin_interval_with_privacy(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
        privacy: Privacy,
    ) -> OSSignpostInterval {
        self.emit(signpost_kind::INTERVAL_BEGIN, name, id, message, privacy);
        OSSignpostInterval {
            id,
            name,
            animation: false,
        }
    }

    #[must_use]
    pub fn begin_animation_interval(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
    ) -> OSSignpostInterval {
        self.begin_animation_interval_with_privacy(name, id, message, Privacy::Private)
    }

    #[must_use]
    pub fn begin_animation_interval_with_privacy(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
        privacy: Privacy,
    ) -> OSSignpostInterval {
        self.emit(
            signpost_kind::ANIMATION_INTERVAL_BEGIN,
            name,
            id,
            message,
            privacy,
        );
        OSSignpostInterval {
            id,
            name,
            animation: true,
        }
    }

    pub fn end_interval(&self, interval: OSSignpostInterval, message: &str) {
        self.end_interval_with_privacy(interval, message, Privacy::Private);
    }

    pub fn end_interval_with_privacy(
        &self,
        interval: OSSignpostInterval,
        message: &str,
        privacy: Privacy,
    ) {
        self.emit(
            signpost_kind::INTERVAL_END,
            interval.name,
            interval.id,
            message,
            privacy,
        );
    }

    pub fn with_interval_signpost<T>(
        &self,
        name: &'static CStr,
        id: OSSignpostId,
        message: &str,
        around: impl FnOnce() -> T,
    ) -> T {
        let interval = self.begin_interval(name, id, message);
        let result = around();
        self.end_interval(interval, message);
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
