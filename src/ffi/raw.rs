#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

use super::{os_activity_t, os_log_t};

extern "C" {
    pub fn apple_log_create(subsystem: *const c_char, category: *const c_char) -> os_log_t;
    pub fn apple_log_default() -> os_log_t;
    pub fn apple_log_disabled() -> os_log_t;
    pub fn apple_log_release(log: os_log_t);
    pub fn apple_log_emit(log: os_log_t, level: i32, message: *const c_char);
    pub fn apple_log_emit_privacy(
        log: os_log_t,
        level: i32,
        message: *const c_char,
        is_public: bool,
    );
    pub fn apple_log_emit_default(level: i32, message: *const c_char);
    pub fn apple_log_emit_default_privacy(level: i32, message: *const c_char, is_public: bool);
    pub fn apple_log_type_enabled(log: os_log_t, level: i32) -> bool;

    pub fn apple_signpost_id_generate(log: os_log_t) -> u64;
    pub fn apple_signpost_id_make_with_pointer(log: os_log_t, ptr: *const c_void) -> u64;
    pub fn apple_signpost_enabled(log: os_log_t) -> bool;
    pub fn apple_signpost_emit(
        log: os_log_t,
        spid: u64,
        kind: i32,
        name: *const c_char,
        message: *const c_char,
        is_public: bool,
    );
    pub fn apple_signpost_event_emit(
        log: os_log_t,
        spid: u64,
        name: *const c_char,
        message: *const c_char,
    );
    pub fn apple_signpost_interval_begin(log: os_log_t, spid: u64, name: *const c_char);
    pub fn apple_signpost_animation_interval_begin(log: os_log_t, spid: u64, name: *const c_char);
    pub fn apple_signpost_interval_end(log: os_log_t, spid: u64, name: *const c_char);

    pub fn apple_activity_get_identifiers(parent_id: *mut u64) -> u64;
    pub fn apple_activity_get_active_id() -> u64;
    pub fn apple_activity_create(
        description: *const c_char,
        parent_activity: os_activity_t,
        flags: u32,
    ) -> os_activity_t;
    pub fn apple_activity_start(description: *const c_char, flags: u32) -> os_activity_t;
    pub fn apple_activity_current() -> os_activity_t;
    pub fn apple_activity_none() -> os_activity_t;
    pub fn apple_activity_null() -> os_activity_t;
    pub fn apple_activity_release(activity: os_activity_t);
    pub fn apple_activity_initiate_f(
        description: *const c_char,
        flags: u32,
        context: *mut c_void,
        function: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub fn apple_activity_apply_f(
        activity: os_activity_t,
        context: *mut c_void,
        function: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub fn apple_activity_scope_enter_alloc(activity: os_activity_t) -> *mut c_void;
    pub fn apple_activity_scope_leave_free(state: *mut c_void);
    pub fn apple_activity_get_identifier(activity: os_activity_t, parent_id: *mut u64) -> u64;
    pub fn apple_activity_end(activity: os_activity_t);
    pub fn apple_activity_label_useraction(label: *const c_char);
    pub fn apple_activity_set_breadcrumb(name: *const c_char);
}
