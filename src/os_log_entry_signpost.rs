use core::ffi::c_void;
use std::ptr::NonNull;

use crate::bridge_support::take_optional_c_string;
use crate::ffi;
use crate::os_log_store::{OSLogEntryCommon, OSLogEntryFromProcess, OSLogEntryWithPayload};

/// The signpost type captured by an `OSLogEntrySignpost`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OSLogEntrySignpostType {
    Undefined,
    IntervalBegin,
    IntervalEnd,
    Event,
}

impl OSLogEntrySignpostType {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::IntervalBegin,
            2 => Self::IntervalEnd,
            3 => Self::Event,
            _ => Self::Undefined,
        }
    }
}

/// Snapshot of an `OSLogEntrySignpost` returned from `OSLogStore`.
pub struct OSLogEntrySignpost {
    ptr: NonNull<c_void>,
}

impl OSLogEntrySignpost {
    pub(crate) const fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self { ptr }
    }

    #[must_use]
    pub fn signpost_identifier(&self) -> u64 {
        unsafe { ffi::apple_log_os_log_entry_signpost_identifier(self.ptr.as_ptr()) }
    }

    #[must_use]
    pub fn signpost_name(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_signpost_name(self.ptr.as_ptr()))
        }
        .unwrap_or_default()
    }

    #[must_use]
    pub fn signpost_type(&self) -> OSLogEntrySignpostType {
        OSLogEntrySignpostType::from_raw(unsafe {
            ffi::apple_log_os_log_entry_signpost_type(self.ptr.as_ptr())
        })
    }
}

impl OSLogEntryCommon for OSLogEntrySignpost {
    fn raw_entry_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl OSLogEntryFromProcess for OSLogEntrySignpost {}
impl OSLogEntryWithPayload for OSLogEntrySignpost {}

impl Drop for OSLogEntrySignpost {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_entry_release(self.ptr.as_ptr()) };
    }
}
