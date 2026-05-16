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

extern "C" {
    pub fn apple_log_create(subsystem: *const c_char, category: *const c_char) -> os_log_t;
    pub fn apple_log_release(log: os_log_t);
    pub fn apple_log_emit(log: os_log_t, level: i32, message: *const c_char);
    pub fn apple_log_emit_default(level: i32, message: *const c_char);

    pub fn apple_log_type_enabled(log: os_log_t, level: i32) -> bool;

    pub fn apple_signpost_id_generate(log: os_log_t) -> u64;
    pub fn apple_signpost_enabled(log: os_log_t) -> bool;
    pub fn apple_signpost_event_emit(
        log: os_log_t,
        spid: u64,
        name: *const c_char,
        message: *const c_char,
    );
    pub fn apple_signpost_interval_begin(log: os_log_t, spid: u64, name: *const c_char);
    pub fn apple_signpost_interval_end(log: os_log_t, spid: u64, name: *const c_char);
}
