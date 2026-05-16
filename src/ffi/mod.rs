//! Raw FFI declarations matching the C shim.

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type os_log_t = *mut c_void;

pub mod level {
    pub const DEFAULT: i32 = 0;
    pub const INFO: i32 = 1;
    pub const DEBUG: i32 = 2;
    pub const ERROR: i32 = 16;
    pub const FAULT: i32 = 17;
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
}
