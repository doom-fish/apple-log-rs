//! Compatibility layer preserving the pre-v0.5 `apple_log::log` API.

pub use crate::logger::{Logger, Privacy};
pub use crate::os_activity::{active_activity_id, active_activity_ids, ActivityIds};
pub use crate::os_log::{
    Level, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING, CATEGORY_POINTS_OF_INTEREST,
};
pub use crate::os_signpost_id::{OSSignpostId, SignpostId};

/// Emits a message through `Logger::default()`.
pub fn log(level: Level, message: &str) {
    Logger::default().log(level, message);
}

/// Emits a message through `Logger::default()` with explicit privacy.
pub fn log_with_privacy(level: Level, message: &str, privacy: Privacy) {
    Logger::default().log_with_privacy(level, message, privacy);
}

/// Returns whether `Logger::default()` enables the requested level.
#[must_use]
pub fn log_enabled(level: Level) -> bool {
    Logger::default().is_enabled(level)
}
