//! Errors returned by the `apple-log` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogError {
    /// An argument contained a NUL byte and could not cross the FFI boundary.
    InvalidArgument(String),
    /// A constructor unexpectedly returned a null handle.
    CreateFailed,
    /// The Swift bridge or wrapped Apple API returned a human-readable error.
    BridgeError(String),
}

impl LogError {
    pub(crate) fn bridge(message: impl Into<String>) -> Self {
        Self::BridgeError(message.into())
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::CreateFailed => write!(f, "bridge constructor returned NULL"),
            Self::BridgeError(message) => write!(f, "bridge error: {message}"),
        }
    }
}

impl std::error::Error for LogError {}
