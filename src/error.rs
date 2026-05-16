//! Errors returned by the `apple-log` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogError {
    /// `subsystem` or `category` contained a NUL byte.
    InvalidArgument(String),
    /// `os_log_create` returned NULL.
    CreateFailed,
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            Self::CreateFailed => write!(f, "os_log_create returned NULL"),
        }
    }
}

impl std::error::Error for LogError {}
