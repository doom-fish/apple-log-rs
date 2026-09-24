#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_panics_doc
)]

use core::ffi::c_void;
use core::ops::{BitOr, BitOrAssign};
use std::path::Path;
use std::ptr::NonNull;
use std::time::{Duration, SystemTime};

use crate::bridge_support::{
    bridge_ptr_result, c_string_arg, path_c_string, secs_to_system_time, system_time_to_secs,
    take_optional_c_string, take_owned_bytes,
};
use crate::error::LogError;
use crate::ffi;
use crate::os_log::Level;
use crate::os_log_entry_activity::OSLogEntryActivity;
use crate::os_log_entry_boundary::OSLogEntryBoundary;
use crate::os_log_entry_log::OSLogEntryLog;
use crate::os_log_entry_signpost::OSLogEntrySignpost;

const LOG_TYPE_DEBUG: u32 = 1 << 0;
const LOG_TYPE_INFO: u32 = 1 << 1;
const LOG_TYPE_DEFAULT: u32 = 1 << 2;
const LOG_TYPE_ERROR: u32 = 1 << 3;
const LOG_TYPE_FAULT: u32 = 1 << 4;

const fn log_type_mask(min_level: Level) -> u32 {
    match min_level {
        Level::Debug => {
            LOG_TYPE_DEBUG | LOG_TYPE_INFO | LOG_TYPE_DEFAULT | LOG_TYPE_ERROR | LOG_TYPE_FAULT
        }
        Level::Info => LOG_TYPE_INFO | LOG_TYPE_DEFAULT | LOG_TYPE_ERROR | LOG_TYPE_FAULT,
        Level::Default => LOG_TYPE_DEFAULT | LOG_TYPE_ERROR | LOG_TYPE_FAULT,
        Level::Error => LOG_TYPE_ERROR | LOG_TYPE_FAULT,
        Level::Fault => LOG_TYPE_FAULT,
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OSLogEntryFilter {
    subsystem: Option<String>,
    category: Option<String>,
    min_level: Option<Level>,
    since: Option<SystemTime>,
    until: Option<SystemTime>,
}

impl OSLogEntryFilter {
    #[must_use]
    pub fn subsystem(mut self, subsystem: impl Into<String>) -> Self {
        self.subsystem = Some(subsystem.into());
        self
    }

    #[must_use]
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    #[must_use]
    pub const fn min_level(mut self, level: Level) -> Self {
        self.min_level = Some(level);
        self
    }

    #[must_use]
    pub const fn since(mut self, start: SystemTime) -> Self {
        self.since = Some(start);
        self
    }

    #[must_use]
    pub const fn until(mut self, end: SystemTime) -> Self {
        self.until = Some(end);
        self
    }
}

struct OSLogEntryList {
    ptr: NonNull<c_void>,
}

impl OSLogEntryList {
    fn into_entries(self) -> Vec<OSLogStoreEntry> {
        let count =
            unsafe { ffi::apple_log_os_log_entry_list_count(self.ptr.as_ptr()) }.max(0) as usize;
        let mut entries = Vec::with_capacity(count);
        for index in 0..count {
            let Some(entry_ptr) = NonNull::new(unsafe {
                ffi::apple_log_os_log_entry_list_get(self.ptr.as_ptr(), index as isize)
            }) else {
                continue;
            };
            let entry = match unsafe { ffi::apple_log_os_log_entry_kind(entry_ptr.as_ptr()) } {
                1 => OSLogStoreEntry::Log(OSLogEntryLog::from_raw(entry_ptr)),
                2 => OSLogStoreEntry::Signpost(OSLogEntrySignpost::from_raw(entry_ptr)),
                3 => OSLogStoreEntry::Boundary(OSLogEntryBoundary::from_raw(entry_ptr)),
                4 => OSLogStoreEntry::Activity(OSLogEntryActivity::from_raw(entry_ptr)),
                _ => {
                    unsafe { ffi::apple_log_os_log_entry_release(entry_ptr.as_ptr()) };
                    continue;
                }
            };
            entries.push(entry);
        }
        entries
    }
}

impl Drop for OSLogEntryList {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_entry_list_release(self.ptr.as_ptr()) };
    }
}

/// `OSLogStore.Scope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OSLogStoreScope {
    System,
    CurrentProcessIdentifier,
}

impl OSLogStoreScope {
    const fn raw(self) -> i32 {
        match self {
            Self::System => 0,
            Self::CurrentProcessIdentifier => 1,
        }
    }
}

/// `OSLogEnumerator.Options`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OSLogEnumeratorOptions(usize);

impl OSLogEnumeratorOptions {
    pub const NONE: Self = Self(0);
    pub const REVERSE: Self = Self(1);

    #[must_use]
    pub const fn bits(self) -> usize {
        self.0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for OSLogEnumeratorOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for OSLogEnumeratorOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// `OSLogEntry.StoreCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OSLogStoreCategory {
    Undefined,
    Metadata,
    ShortTerm,
    LongTermAuto,
    LongTerm1,
    LongTerm3,
    LongTerm7,
    LongTerm14,
    LongTerm30,
}

impl OSLogStoreCategory {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Metadata,
            2 => Self::ShortTerm,
            3 => Self::LongTermAuto,
            4 => Self::LongTerm1,
            5 => Self::LongTerm3,
            6 => Self::LongTerm7,
            7 => Self::LongTerm14,
            8 => Self::LongTerm30,
            _ => Self::Undefined,
        }
    }
}

/// `OSLogMessageComponent.ArgumentCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OSLogMessageArgumentCategory {
    Undefined,
    Data,
    Double,
    Int64,
    String,
    UInt64,
}

impl OSLogMessageArgumentCategory {
    const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Data,
            2 => Self::Double,
            3 => Self::Int64,
            4 => Self::String,
            5 => Self::UInt64,
            _ => Self::Undefined,
        }
    }
}

/// Decoded `OSLogMessageComponent.Argument` value.
#[derive(Debug, Clone, PartialEq)]
pub enum OSLogMessageArgument {
    Undefined,
    Data(Vec<u8>),
    Double(f64),
    Signed(i64),
    String(String),
    Unsigned(u64),
}

/// Snapshot of an `OSLogMessageComponent`.
pub struct OSLogMessageComponent {
    ptr: NonNull<c_void>,
}

impl OSLogMessageComponent {
    #[must_use]
    pub fn format_substring(&self) -> String {
        unsafe {
            take_optional_c_string(
                ffi::apple_log_os_log_message_component_copy_format_substring(self.ptr.as_ptr()),
            )
        }
        .unwrap_or_default()
    }

    #[must_use]
    pub fn placeholder(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_message_component_copy_placeholder(
                self.ptr.as_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    pub fn argument_category(&self) -> OSLogMessageArgumentCategory {
        OSLogMessageArgumentCategory::from_raw(unsafe {
            ffi::apple_log_os_log_message_component_get_argument_category(self.ptr.as_ptr())
        })
    }

    #[must_use]
    pub fn argument(&self) -> OSLogMessageArgument {
        match self.argument_category() {
            OSLogMessageArgumentCategory::Undefined => OSLogMessageArgument::Undefined,
            OSLogMessageArgumentCategory::Data => {
                let mut length = 0_isize;
                let bytes = unsafe {
                    take_owned_bytes(
                        ffi::apple_log_os_log_message_component_copy_data(
                            self.ptr.as_ptr(),
                            &raw mut length,
                        ),
                        length.max(0) as usize,
                    )
                };
                OSLogMessageArgument::Data(bytes)
            }
            OSLogMessageArgumentCategory::Double => OSLogMessageArgument::Double(unsafe {
                ffi::apple_log_os_log_message_component_get_double(self.ptr.as_ptr())
            }),
            OSLogMessageArgumentCategory::Int64 => OSLogMessageArgument::Signed(unsafe {
                ffi::apple_log_os_log_message_component_get_int64(self.ptr.as_ptr())
            }),
            OSLogMessageArgumentCategory::String => {
                let value = unsafe {
                    take_optional_c_string(ffi::apple_log_os_log_message_component_copy_string(
                        self.ptr.as_ptr(),
                    ))
                }
                .unwrap_or_default();
                OSLogMessageArgument::String(value)
            }
            OSLogMessageArgumentCategory::UInt64 => OSLogMessageArgument::Unsigned(unsafe {
                ffi::apple_log_os_log_message_component_get_uint64(self.ptr.as_ptr())
            }),
        }
    }
}

impl Drop for OSLogMessageComponent {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_message_component_release(self.ptr.as_ptr()) };
    }
}

/// Common fields exposed by every `OSLogEntry` snapshot.
pub trait OSLogEntryCommon {
    fn raw_entry_ptr(&self) -> *mut c_void;

    #[must_use]
    fn composed_message(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_composed_message(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn date(&self) -> SystemTime {
        secs_to_system_time(unsafe {
            ffi::apple_log_os_log_entry_get_date_seconds(self.raw_entry_ptr())
        })
    }

    #[must_use]
    fn store_category(&self) -> OSLogStoreCategory {
        OSLogStoreCategory::from_raw(unsafe {
            ffi::apple_log_os_log_entry_get_store_category(self.raw_entry_ptr())
        })
    }
}

/// Fields shared by entries conforming to `OSLogEntryFromProcess`.
pub trait OSLogEntryFromProcess: OSLogEntryCommon {
    #[must_use]
    fn activity_identifier(&self) -> u64 {
        unsafe { ffi::apple_log_os_log_entry_get_activity_identifier(self.raw_entry_ptr()) }
    }

    #[must_use]
    fn process(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_process(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn process_identifier(&self) -> i32 {
        unsafe { ffi::apple_log_os_log_entry_get_process_identifier(self.raw_entry_ptr()) }
    }

    #[must_use]
    fn sender(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_sender(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn thread_identifier(&self) -> u64 {
        unsafe { ffi::apple_log_os_log_entry_get_thread_identifier(self.raw_entry_ptr()) }
    }
}

/// Fields shared by entries conforming to `OSLogEntryWithPayload`.
pub trait OSLogEntryWithPayload: OSLogEntryFromProcess {
    #[must_use]
    fn category(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_category(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn format_string(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_format_string(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn subsystem(&self) -> String {
        unsafe {
            take_optional_c_string(ffi::apple_log_os_log_entry_copy_subsystem(
                self.raw_entry_ptr(),
            ))
        }
        .unwrap_or_default()
    }

    #[must_use]
    fn components(&self) -> Vec<OSLogMessageComponent> {
        let count = unsafe { ffi::apple_log_os_log_entry_component_count(self.raw_entry_ptr()) };
        (0..count.max(0) as usize)
            .filter_map(|index| {
                NonNull::new(unsafe {
                    ffi::apple_log_os_log_entry_component_get(self.raw_entry_ptr(), index as isize)
                })
                .map(|ptr| OSLogMessageComponent { ptr })
            })
            .collect()
    }
}

/// `OSLogPosition` wrapper.
pub struct OSLogPosition {
    ptr: NonNull<c_void>,
}

impl OSLogPosition {
    const fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self { ptr }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl Drop for OSLogPosition {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_position_release(self.ptr.as_ptr()) };
    }
}

/// Typed entry returned from `OSLogStore` enumeration.
pub enum OSLogStoreEntry {
    Log(OSLogEntryLog),
    Signpost(OSLogEntrySignpost),
    Boundary(OSLogEntryBoundary),
    Activity(OSLogEntryActivity),
}

/// Safe wrapper around `OSLogStore`.
pub struct OSLogStore {
    ptr: NonNull<c_void>,
}

impl OSLogStore {
    /// Opens the local log store.
    ///
    /// # Errors
    ///
    /// Returns an error if the store cannot be opened.
    pub fn local() -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSLogStore::local", |error_out| unsafe {
            ffi::apple_log_os_log_store_local(error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Opens a scoped log store.
    ///
    /// # Errors
    ///
    /// Returns an error if the store cannot be opened.
    pub fn new(scope: OSLogStoreScope) -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSLogStore::new", |error_out| unsafe {
            ffi::apple_log_os_log_store_create(scope.raw(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Opens a logarchive URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the path contains a NUL byte or the store cannot be opened.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Self, LogError> {
        let path = path_c_string(path.as_ref())?;
        let ptr = bridge_ptr_result("OSLogStore::from_url", |error_out| unsafe {
            ffi::apple_log_os_log_store_from_url(path.as_ptr(), error_out)
        })?;
        Ok(Self { ptr })
    }

    #[must_use]
    pub fn position_at(&self, time: SystemTime) -> OSLogPosition {
        let ptr = NonNull::new(unsafe {
            ffi::apple_log_os_log_store_position_date(self.ptr.as_ptr(), system_time_to_secs(time))
        })
        .expect("Swift bridge never returns NULL for OSLogStore::position_at");
        OSLogPosition::from_raw(ptr)
    }

    #[must_use]
    pub fn position_time_interval_since_end(&self, duration: Duration) -> OSLogPosition {
        let ptr = NonNull::new(unsafe {
            ffi::apple_log_os_log_store_position_since_end(
                self.ptr.as_ptr(),
                duration.as_secs_f64(),
            )
        })
        .expect("Swift bridge never returns NULL for OSLogStore::position_time_interval_since_end");
        OSLogPosition::from_raw(ptr)
    }

    #[must_use]
    pub fn position_time_interval_since_latest_boot(&self, duration: Duration) -> OSLogPosition {
        let ptr = NonNull::new(unsafe {
            ffi::apple_log_os_log_store_position_since_latest_boot(
                self.ptr.as_ptr(),
                duration.as_secs_f64(),
            )
        })
        .expect(
            "Swift bridge never returns NULL for OSLogStore::position_time_interval_since_latest_boot",
        );
        OSLogPosition::from_raw(ptr)
    }

    /// Enumerates entries from the store.
    ///
    /// # Errors
    ///
    /// Returns an error if a filter string contains a NUL byte or the store enumeration fails.
    pub fn get_entries(
        &self,
        options: OSLogEnumeratorOptions,
        position: Option<&OSLogPosition>,
        filter: &OSLogEntryFilter,
        max_entries: usize,
    ) -> Result<Vec<OSLogStoreEntry>, LogError> {
        let subsystem = filter
            .subsystem
            .as_deref()
            .map(|value| c_string_arg("subsystem", value))
            .transpose()?;
        let category = filter
            .category
            .as_deref()
            .map(|value| c_string_arg("category", value))
            .transpose()?;
        let list = OSLogEntryList {
            ptr: bridge_ptr_result("OSLogStore::get_entries", |error_out| unsafe {
                ffi::apple_log_os_log_store_get_entries(
                    self.ptr.as_ptr(),
                    options.bits(),
                    position.map_or(std::ptr::null_mut(), OSLogPosition::as_ptr),
                    subsystem
                        .as_ref()
                        .map_or(std::ptr::null(), |value| value.as_ptr()),
                    category
                        .as_ref()
                        .map_or(std::ptr::null(), |value| value.as_ptr()),
                    filter.min_level.map_or(0, log_type_mask),
                    filter.since.is_some(),
                    filter.since.map_or(0.0, system_time_to_secs),
                    filter.until.is_some(),
                    filter.until.map_or(0.0, system_time_to_secs),
                    max_entries,
                    error_out,
                )
            })?,
        };
        Ok(list.into_entries())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn get_entries_with_predicate(
        &self,
        options: OSLogEnumeratorOptions,
        position: Option<&OSLogPosition>,
        predicate_format: &str,
        max_entries: usize,
    ) -> Result<Vec<OSLogStoreEntry>, LogError> {
        let predicate_format = c_string_arg("predicate", predicate_format)?;
        let list = OSLogEntryList {
            ptr: bridge_ptr_result(
                "OSLogStore::get_entries_with_predicate",
                |error_out| unsafe {
                    ffi::apple_log_os_log_store_get_entries_with_predicate(
                        self.ptr.as_ptr(),
                        options.bits(),
                        position.map_or(std::ptr::null_mut(), OSLogPosition::as_ptr),
                        predicate_format.as_ptr(),
                        max_entries,
                        error_out,
                    )
                },
            )?,
        };
        Ok(list.into_entries())
    }
}

impl Drop for OSLogStore {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_log_store_release(self.ptr.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_level_selects_that_level_and_everything_more_severe() {
        assert_eq!(log_type_mask(Level::Fault), LOG_TYPE_FAULT);
        assert_eq!(log_type_mask(Level::Error), LOG_TYPE_ERROR | LOG_TYPE_FAULT);
        assert_eq!(
            log_type_mask(Level::Default),
            LOG_TYPE_DEFAULT | LOG_TYPE_ERROR | LOG_TYPE_FAULT
        );
        assert_eq!(
            log_type_mask(Level::Info),
            LOG_TYPE_INFO | LOG_TYPE_DEFAULT | LOG_TYPE_ERROR | LOG_TYPE_FAULT
        );
        assert_eq!(log_type_mask(Level::Debug), 0b1_1111);
    }

    #[test]
    fn filter_builder_records_every_field() {
        let start = std::time::UNIX_EPOCH + Duration::from_secs(10);
        let end = std::time::UNIX_EPOCH + Duration::from_secs(20);
        let filter = OSLogEntryFilter::default()
            .subsystem("fish.doom")
            .category("net")
            .min_level(Level::Error)
            .since(start)
            .until(end);
        assert_eq!(filter.subsystem.as_deref(), Some("fish.doom"));
        assert_eq!(filter.category.as_deref(), Some("net"));
        assert_eq!(filter.min_level, Some(Level::Error));
        assert_eq!(filter.since, Some(start));
        assert_eq!(filter.until, Some(end));
        assert_eq!(OSLogEntryFilter::default().min_level, None);
    }
}
