use core::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;
use crate::os_log_store::{OSLogEntryCommon, OSLogEntryFromProcess, OSLogEntryWithPayload};

/// The severity of an `OSLogEntryLog`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OSLogEntryLogLevel {
    Undefined,
    Debug,
    Info,
    Notice,
    Error,
    Fault,
}

impl OSLogEntryLogLevel {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Notice,
            4 => Self::Error,
            5 => Self::Fault,
            _ => Self::Undefined,
        }
    }
}

/// Snapshot of an `OSLogEntryLog` returned from `OSLogStore`.
pub struct OSLogEntryLog {
    ptr: NonNull<c_void>,
}

impl OSLogEntryLog {
    pub(crate) const fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self { ptr }
    }

    #[must_use]
    pub fn level(&self) -> OSLogEntryLogLevel {
        OSLogEntryLogLevel::from_raw(unsafe { ffi::apple_log_os_log_entry_level(self.ptr.as_ptr()) })
    }
}

impl OSLogEntryCommon for OSLogEntryLog {
    fn raw_entry_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl OSLogEntryFromProcess for OSLogEntryLog {}
impl OSLogEntryWithPayload for OSLogEntryLog {}

impl Drop for OSLogEntryLog {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_entry_release(self.ptr.as_ptr()) };
    }
}
