use crate::ffi;
use crate::os_log::OSLog;

/// Opaque signpost identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSSignpostId(u64);

/// Backwards-compatible alias for the pre-v0.5 signpost type.
pub type SignpostId = OSSignpostId;

impl OSSignpostId {
    pub const NULL: Self = Self(ffi::signpost_id::NULL);
    pub const INVALID: Self = Self(ffi::signpost_id::INVALID);
    pub const EXCLUSIVE: Self = Self(ffi::signpost_id::EXCLUSIVE);

    #[must_use]
    pub fn generate(log: &OSLog) -> Self {
        Self(unsafe { ffi::apple_log_os_signpost_id_generate(log.as_ptr()) })
    }

    #[must_use]
    pub fn from_pointer<T>(log: &OSLog, pointer: *const T) -> Self {
        Self(unsafe {
            ffi::apple_log_os_signpost_id_make_with_pointer(log.as_ptr(), pointer.cast())
        })
    }

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_u64(raw: u64) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0 == Self::NULL.0
    }

    #[must_use]
    pub const fn is_invalid(self) -> bool {
        self.0 == Self::INVALID.0
    }
}
