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
    pub fn apple_activity_release(activity: os_activity_t);
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

    pub fn apple_os_atomic_queue_create() -> *mut c_void;
    pub fn apple_os_atomic_queue_destroy(queue: *mut c_void);
    pub fn apple_os_atomic_queue_enqueue_value(queue: *mut c_void, value: usize);
    pub fn apple_os_atomic_queue_dequeue_value(queue: *mut c_void, value_out: *mut usize) -> bool;
    pub fn apple_os_atomic_fifo_queue_create() -> *mut c_void;
    pub fn apple_os_atomic_fifo_queue_destroy(queue: *mut c_void);
    pub fn apple_os_atomic_fifo_enqueue_value(queue: *mut c_void, value: usize);
    pub fn apple_os_atomic_fifo_dequeue_value(queue: *mut c_void, value_out: *mut usize) -> bool;

    pub fn apple_os_atomic_add32(amount: i32, value: *mut i32) -> i32;
    pub fn apple_os_atomic_add32_barrier(amount: i32, value: *mut i32) -> i32;
    pub fn apple_os_atomic_increment32(value: *mut i32) -> i32;
    pub fn apple_os_atomic_increment32_barrier(value: *mut i32) -> i32;
    pub fn apple_os_atomic_decrement32(value: *mut i32) -> i32;
    pub fn apple_os_atomic_decrement32_barrier(value: *mut i32) -> i32;
    pub fn apple_os_atomic_or32(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_or32_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_or32_orig(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_or32_orig_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_and32(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_and32_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_and32_orig(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_and32_orig_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_xor32(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_xor32_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_xor32_orig(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_xor32_orig_barrier(mask: u32, value: *mut u32) -> i32;
    pub fn apple_os_atomic_compare_and_swap32(old_value: i32, new_value: i32, value: *mut i32) -> bool;
    pub fn apple_os_atomic_compare_and_swap32_barrier(
        old_value: i32,
        new_value: i32,
        value: *mut i32,
    ) -> bool;
    pub fn apple_os_atomic_add64(amount: i64, value: *mut i64) -> i64;
    pub fn apple_os_atomic_add64_barrier(amount: i64, value: *mut i64) -> i64;
    pub fn apple_os_atomic_increment64(value: *mut i64) -> i64;
    pub fn apple_os_atomic_increment64_barrier(value: *mut i64) -> i64;
    pub fn apple_os_atomic_decrement64(value: *mut i64) -> i64;
    pub fn apple_os_atomic_decrement64_barrier(value: *mut i64) -> i64;
    pub fn apple_os_atomic_compare_and_swap64(old_value: i64, new_value: i64, value: *mut i64) -> bool;
    pub fn apple_os_atomic_compare_and_swap64_barrier(
        old_value: i64,
        new_value: i64,
        value: *mut i64,
    ) -> bool;
    pub fn apple_os_atomic_compare_and_swap_int(old_value: i32, new_value: i32, value: *mut i32)
        -> bool;
    pub fn apple_os_atomic_compare_and_swap_int_barrier(
        old_value: i32,
        new_value: i32,
        value: *mut i32,
    ) -> bool;
    pub fn apple_os_atomic_compare_and_swap_long(old_value: i64, new_value: i64, value: *mut i64)
        -> bool;
    pub fn apple_os_atomic_compare_and_swap_long_barrier(
        old_value: i64,
        new_value: i64,
        value: *mut i64,
    ) -> bool;
    pub fn apple_os_atomic_compare_and_swap_ptr(
        old_value: *mut c_void,
        new_value: *mut c_void,
        value: *mut *mut c_void,
    ) -> bool;
    pub fn apple_os_atomic_compare_and_swap_ptr_barrier(
        old_value: *mut c_void,
        new_value: *mut c_void,
        value: *mut *mut c_void,
    ) -> bool;
    pub fn apple_os_atomic_test_and_set(bit: u32, address: *mut c_void) -> bool;
    pub fn apple_os_atomic_test_and_set_barrier(bit: u32, address: *mut c_void) -> bool;
    pub fn apple_os_atomic_test_and_clear(bit: u32, address: *mut c_void) -> bool;
    pub fn apple_os_atomic_test_and_clear_barrier(bit: u32, address: *mut c_void) -> bool;
}
