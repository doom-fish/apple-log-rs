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
pub use log::{active_activity_id, log, Level, Logger, SignpostId};

/// Common imports.
pub mod prelude {
    pub use crate::error::LogError;
    pub use crate::log::{active_activity_id, log, Level, Logger, SignpostId};
}
