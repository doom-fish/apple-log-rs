#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's [`os_log`](https://developer.apple.com/documentation/os/logging)
//! on macOS — structured logging that integrates with Console.app and
//! the `log` CLI.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod log;

pub use error::LogError;
pub use log::{
    active_activity_id, active_activity_ids, log, log_enabled, log_with_privacy, ActivityIds,
    Level, Logger, Privacy, SignpostId, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING,
    CATEGORY_POINTS_OF_INTEREST,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::LogError;
    pub use crate::log::{
        active_activity_id, active_activity_ids, log, log_enabled, log_with_privacy, ActivityIds,
        Level, Logger, Privacy, SignpostId, CATEGORY_DYNAMIC_STACK_TRACING,
        CATEGORY_DYNAMIC_TRACING, CATEGORY_POINTS_OF_INTEREST,
    };
}
