//! Compatibility layer preserving the pre-v0.5 `apple_log::log` API.

use crate::bridge_support::sanitized_c_string;
use crate::ffi;
pub use crate::logger::{Logger, Privacy};
pub use crate::os_activity::{active_activity_id, active_activity_ids, ActivityIds};
pub use crate::os_log::{
    Level, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING, CATEGORY_POINTS_OF_INTEREST,
};
pub use crate::os_signpost_id::{OSSignpostId, SignpostId};

/// Emits a message through `Logger::default()`.
pub fn log(level: Level, message: &str) {
    log_with_privacy(level, message, Privacy::Private);
}

/// Emits a message through `Logger::default()` with explicit privacy.
pub fn log_with_privacy(level: Level, message: &str, privacy: Privacy) {
    let message = sanitized_c_string(message);
    unsafe {
        ffi::default_log_emit(
            i32::from(level as u8),
            message.as_ptr(),
            privacy == Privacy::Public,
        );
    }
}

/// Returns whether `Logger::default()` enables the requested level.
#[must_use]
pub fn log_enabled(level: Level) -> bool {
    unsafe { ffi::default_log_type_enabled(std::ptr::null_mut(), i32::from(level as u8)) }
}
