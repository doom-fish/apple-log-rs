//! Bridge and optional raw FFI declarations.

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type os_log_t = *mut c_void;
pub type os_activity_t = *mut c_void;

pub mod level {
    pub const DEFAULT: u8 = 0;
    pub const INFO: u8 = 1;
    pub const DEBUG: u8 = 2;
    pub const ERROR: u8 = 16;
    pub const FAULT: u8 = 17;
}

pub mod category {
    pub const POINTS_OF_INTEREST: &str = "PointsOfInterest";
    pub const DYNAMIC_TRACING: &str = "DynamicTracing";
    pub const DYNAMIC_STACK_TRACING: &str = "DynamicStackTracing";
}

pub mod signpost_id {
    pub const NULL: u64 = 0;
    pub const INVALID: u64 = u64::MAX;
    pub const EXCLUSIVE: u64 = 0xEEEE_B0B5_B2B2_EEEE;
}

pub mod signpost_kind {
    pub const EVENT: i32 = 0;
    pub const INTERVAL_BEGIN: i32 = 1;
    pub const ANIMATION_INTERVAL_BEGIN: i32 = 2;
    pub const INTERVAL_END: i32 = 3;
}

extern "C" {
    #[link_name = "apple_log_emit_default_privacy"]
    pub(crate) fn default_log_emit(level: i32, message: *const c_char, is_public: bool);
    #[link_name = "apple_log_type_enabled"]
    pub(crate) fn default_log_type_enabled(log: *mut c_void, level: i32) -> bool;

    pub(crate) fn apple_log_string_free(string: *mut c_char);
    pub(crate) fn apple_log_bytes_free(bytes: *mut c_void);

    pub(crate) fn apple_log_os_log_create(
        subsystem: *const c_char,
        category: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_default() -> *mut c_void;
    pub(crate) fn apple_log_os_log_disabled() -> *mut c_void;
    pub(crate) fn apple_log_os_log_release(log: *mut c_void);
    pub(crate) fn apple_log_os_log_is_enabled(log: *mut c_void, level: u8) -> bool;
    pub(crate) fn apple_log_os_log_signposts_enabled(log: *mut c_void) -> bool;
    pub(crate) fn apple_log_os_log_copy_subsystem(log: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_copy_category(log: *mut c_void) -> *mut c_char;

    pub(crate) fn apple_log_logger_create(
        subsystem: *const c_char,
        category: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_logger_default() -> *mut c_void;
    pub(crate) fn apple_log_logger_disabled() -> *mut c_void;
    pub(crate) fn apple_log_logger_from_os_log(
        log: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_logger_release(logger: *mut c_void);
    pub(crate) fn apple_log_logger_log(
        logger: *mut c_void,
        severity: i32,
        privacy: i32,
        message: *const c_char,
    );
    pub(crate) fn apple_log_logger_is_enabled(logger: *mut c_void, level: u8) -> bool;
    pub(crate) fn apple_log_logger_signpost_id(logger: *mut c_void) -> u64;
    pub(crate) fn apple_log_logger_signpost_id_from_pointer(
        logger: *mut c_void,
        pointer: *const c_void,
    ) -> u64;
    pub(crate) fn apple_log_logger_signposts_enabled(logger: *mut c_void) -> bool;
    pub(crate) fn apple_log_logger_signpost_emit(
        logger: *mut c_void,
        signpost_id: u64,
        kind: i32,
        name: *const c_char,
        message: *const c_char,
        is_public: bool,
    );

    pub(crate) fn apple_log_os_signpost_id_generate(log: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_signpost_id_make_with_pointer(
        log: *mut c_void,
        pointer: *const c_void,
    ) -> u64;

    pub(crate) fn apple_log_os_signposter_create(
        subsystem: *const c_char,
        category: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_signposter_default() -> *mut c_void;
    pub(crate) fn apple_log_os_signposter_disabled() -> *mut c_void;
    pub(crate) fn apple_log_os_signposter_from_os_log(
        log: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_signposter_from_logger(
        logger: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_signposter_release(signposter: *mut c_void);
    pub(crate) fn apple_log_os_signposter_is_enabled(signposter: *mut c_void) -> bool;
    pub(crate) fn apple_log_os_signposter_make_signpost_id(signposter: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_signposter_make_signpost_id_from_pointer(
        signposter: *mut c_void,
        pointer: *const c_void,
    ) -> u64;
    pub(crate) fn apple_log_os_signposter_emit(
        signposter: *mut c_void,
        signpost_id: u64,
        kind: i32,
        name: *const c_char,
        message: *const c_char,
        is_public: bool,
    );

    pub(crate) fn apple_log_os_activity_create(
        description: *const c_char,
        parent: *mut c_void,
        flags: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_activity_start(
        description: *const c_char,
        flags: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_activity_current() -> *mut c_void;
    pub(crate) fn apple_log_os_activity_none() -> *mut c_void;
    pub(crate) fn apple_log_os_activity_null() -> *mut c_void;
    pub(crate) fn apple_log_os_activity_release(activity: *mut c_void);
    pub(crate) fn apple_log_os_activity_get_identifier(
        activity: *mut c_void,
        parent_out: *mut u64,
    ) -> u64;
    pub(crate) fn apple_log_os_activity_initiate(
        description: *const c_char,
        flags: u32,
        context: *mut c_void,
        function: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub(crate) fn apple_log_os_activity_initiate_f(
        description: *const c_char,
        flags: u32,
        context: *mut c_void,
        function: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub(crate) fn apple_log_os_activity_apply(
        activity: *mut c_void,
        context: *mut c_void,
        function: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub(crate) fn apple_log_os_activity_scope_enter(activity: *mut c_void) -> *mut c_void;
    pub(crate) fn apple_log_os_activity_scope_leave(state: *mut c_void);
    pub(crate) fn apple_log_os_activity_label_useraction(label: *const c_char);
    pub(crate) fn apple_log_os_activity_set_breadcrumb(name: *const c_char);
    pub(crate) fn apple_log_os_activity_end(activity: *mut c_void);
    pub(crate) fn apple_log_os_activity_get_active_identifiers(parent_out: *mut u64) -> u64;

    pub(crate) fn apple_log_os_log_store_local(error_out: *mut *mut c_char) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_create(
        scope: i32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_from_url(
        path: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_release(store: *mut c_void);
    pub(crate) fn apple_log_os_log_store_position_date(
        store: *mut c_void,
        seconds_since_1970: f64,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_position_since_end(
        store: *mut c_void,
        seconds: f64,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_position_since_latest_boot(
        store: *mut c_void,
        seconds: f64,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_position_release(position: *mut c_void);
    pub(crate) fn apple_log_os_log_store_get_entries(
        store: *mut c_void,
        options: usize,
        position: *mut c_void,
        subsystem: *const c_char,
        category: *const c_char,
        log_type_mask: u32,
        has_start: bool,
        start_seconds: f64,
        has_end: bool,
        end_seconds: f64,
        max_entries: usize,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_store_get_entries_with_predicate(
        store: *mut c_void,
        options: usize,
        position: *mut c_void,
        predicate_format: *const c_char,
        max_entries: usize,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_entry_list_release(list: *mut c_void);
    pub(crate) fn apple_log_os_log_entry_list_count(list: *mut c_void) -> isize;
    pub(crate) fn apple_log_os_log_entry_list_get(list: *mut c_void, index: isize) -> *mut c_void;
    pub(crate) fn apple_log_os_log_entry_release(entry: *mut c_void);
    pub(crate) fn apple_log_os_log_entry_kind(entry: *mut c_void) -> i32;
    pub(crate) fn apple_log_os_log_entry_copy_composed_message(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_get_date_seconds(entry: *mut c_void) -> f64;
    pub(crate) fn apple_log_os_log_entry_get_store_category(entry: *mut c_void) -> i32;
    pub(crate) fn apple_log_os_log_entry_get_activity_identifier(entry: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_log_entry_copy_process(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_get_process_identifier(entry: *mut c_void) -> i32;
    pub(crate) fn apple_log_os_log_entry_copy_sender(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_get_thread_identifier(entry: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_log_entry_copy_category(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_copy_format_string(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_copy_subsystem(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_component_count(entry: *mut c_void) -> isize;
    pub(crate) fn apple_log_os_log_entry_component_get(
        entry: *mut c_void,
        index: isize,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_message_component_release(component: *mut c_void);
    pub(crate) fn apple_log_os_log_message_component_copy_format_substring(
        component: *mut c_void,
    ) -> *mut c_char;
    pub(crate) fn apple_log_os_log_message_component_copy_placeholder(
        component: *mut c_void,
    ) -> *mut c_char;
    pub(crate) fn apple_log_os_log_message_component_get_argument_category(
        component: *mut c_void,
    ) -> i32;
    pub(crate) fn apple_log_os_log_message_component_get_double(component: *mut c_void) -> f64;
    pub(crate) fn apple_log_os_log_message_component_get_int64(component: *mut c_void) -> i64;
    pub(crate) fn apple_log_os_log_message_component_copy_string(
        component: *mut c_void,
    ) -> *mut c_char;
    pub(crate) fn apple_log_os_log_message_component_get_uint64(component: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_log_message_component_copy_data(
        component: *mut c_void,
        length_out: *mut isize,
    ) -> *mut c_void;
    pub(crate) fn apple_log_os_log_entry_level(entry: *mut c_void) -> i32;
    pub(crate) fn apple_log_os_log_entry_signpost_identifier(entry: *mut c_void) -> u64;
    pub(crate) fn apple_log_os_log_entry_copy_signpost_name(entry: *mut c_void) -> *mut c_char;
    pub(crate) fn apple_log_os_log_entry_signpost_type(entry: *mut c_void) -> i32;
    pub(crate) fn apple_log_os_log_entry_is_boundary(entry: *mut c_void) -> bool;
    pub(crate) fn apple_log_os_log_entry_parent_activity_identifier(entry: *mut c_void) -> u64;
}

#[cfg(feature = "raw-ffi")]
pub mod raw;
#[cfg(feature = "raw-ffi")]
pub use raw::*;
