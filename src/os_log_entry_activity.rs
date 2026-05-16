use core::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;
use crate::os_log_store::{OSLogEntryCommon, OSLogEntryFromProcess};

/// Snapshot of an `OSLogEntryActivity` returned from `OSLogStore`.
pub struct OSLogEntryActivity {
    ptr: NonNull<c_void>,
}

impl OSLogEntryActivity {
    pub(crate) const fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self { ptr }
    }

    #[must_use]
    pub fn parent_activity_identifier(&self) -> u64 {
        unsafe { ffi::apple_log_os_log_entry_parent_activity_identifier(self.ptr.as_ptr()) }
    }
}

impl OSLogEntryCommon for OSLogEntryActivity {
    fn raw_entry_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl OSLogEntryFromProcess for OSLogEntryActivity {}

impl Drop for OSLogEntryActivity {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_entry_release(self.ptr.as_ptr()) };
    }
}
