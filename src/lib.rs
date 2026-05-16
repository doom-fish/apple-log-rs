#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod ffi;
pub mod logger;
pub mod log;
pub mod os_activity;
pub mod os_atomic;
pub mod os_log;
pub mod os_log_entry_activity;
pub mod os_log_entry_boundary;
pub mod os_log_entry_log;
pub mod os_log_entry_signpost;
pub mod os_log_store;
pub mod os_signpost_id;
pub mod os_signposter;

mod bridge_support;

pub use error::LogError;
pub use log::{active_activity_id, active_activity_ids, log, log_enabled, log_with_privacy, ActivityIds};
pub use logger::{Logger, Privacy};
pub use os_activity::{OSActivity, OSActivityFlags, OSActivityScope};
pub use os_atomic::{OSAtomicFifoQueue, OSAtomicI32, OSAtomicI64, OSAtomicQueue};
pub use os_log::{
    Level, OSLog, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING,
    CATEGORY_POINTS_OF_INTEREST,
};
pub use os_log_entry_activity::OSLogEntryActivity;
pub use os_log_entry_boundary::OSLogEntryBoundary;
pub use os_log_entry_log::{OSLogEntryLog, OSLogEntryLogLevel};
pub use os_log_entry_signpost::{OSLogEntrySignpost, OSLogEntrySignpostType};
pub use os_log_store::{
    OSLogEntryCommon, OSLogEntryFromProcess, OSLogEntryWithPayload, OSLogEnumeratorOptions,
    OSLogMessageArgument, OSLogMessageArgumentCategory, OSLogMessageComponent, OSLogPosition,
    OSLogStore, OSLogStoreCategory, OSLogStoreEntry, OSLogStoreScope,
};
pub use os_signpost_id::{OSSignpostId, SignpostId};
pub use os_signposter::{OSSignpostInterval, OSSignposter};

/// Common imports.
pub mod prelude {
    pub use crate::error::LogError;
    pub use crate::log::{
        active_activity_id, active_activity_ids, log, log_enabled, log_with_privacy, ActivityIds,
    };
    pub use crate::logger::{Logger, Privacy};
    pub use crate::os_activity::{OSActivity, OSActivityFlags, OSActivityScope};
    pub use crate::os_atomic::{OSAtomicFifoQueue, OSAtomicI32, OSAtomicI64, OSAtomicQueue};
    pub use crate::os_log::{
        Level, OSLog, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING,
        CATEGORY_POINTS_OF_INTEREST,
    };
    pub use crate::os_log_entry_activity::OSLogEntryActivity;
    pub use crate::os_log_entry_boundary::OSLogEntryBoundary;
    pub use crate::os_log_entry_log::{OSLogEntryLog, OSLogEntryLogLevel};
    pub use crate::os_log_entry_signpost::{OSLogEntrySignpost, OSLogEntrySignpostType};
    pub use crate::os_log_store::{
        OSLogEntryCommon, OSLogEntryFromProcess, OSLogEntryWithPayload, OSLogEnumeratorOptions,
        OSLogMessageArgument, OSLogMessageArgumentCategory, OSLogMessageComponent, OSLogPosition,
        OSLogStore, OSLogStoreCategory, OSLogStoreEntry, OSLogStoreScope,
    };
    pub use crate::os_signpost_id::{OSSignpostId, SignpostId};
    pub use crate::os_signposter::{OSSignpostInterval, OSSignposter};
}
