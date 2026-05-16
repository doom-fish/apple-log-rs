use core::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;
use crate::os_log_store::OSLogEntryCommon;

/// Snapshot of an `OSLogEntryBoundary` returned from `OSLogStore`.
pub struct OSLogEntryBoundary {
    ptr: NonNull<c_void>,
}

impl OSLogEntryBoundary {
    pub(crate) const fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self { ptr }
    }

    #[must_use]
    pub fn is_boundary(&self) -> bool {
        unsafe { ffi::apple_log_os_log_entry_is_boundary(self.ptr.as_ptr()) }
    }
}

impl OSLogEntryCommon for OSLogEntryBoundary {
    fn raw_entry_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl Drop for OSLogEntryBoundary {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_entry_release(self.ptr.as_ptr()) };
    }
}
