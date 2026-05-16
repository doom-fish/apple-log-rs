#![allow(clippy::missing_panics_doc, clippy::must_use_candidate)]

use core::ffi::c_void;
use std::ptr::NonNull;

use crate::bridge_support::bridge_ptr_result;
use crate::error::LogError;
use crate::ffi;

/// Safe wrapper around a 32-bit legacy `OSAtomic` value.
pub struct OSAtomicI32 {
    ptr: NonNull<c_void>,
}

impl OSAtomicI32 {
    #[must_use]
    pub fn new(value: i32) -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_atomic_i32_new(value) })
                .expect("Swift bridge never returns NULL for OSAtomicI32::new"),
        }
    }

    #[must_use]
    pub fn load(&self) -> i32 {
        unsafe { ffi::apple_log_os_atomic_i32_load(self.ptr.as_ptr()) }
    }

    pub fn store(&self, value: i32) {
        unsafe { ffi::apple_log_os_atomic_i32_store(self.ptr.as_ptr(), value) };
    }

    pub fn add(&self, amount: i32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_add(self.ptr.as_ptr(), amount) } }
    pub fn add_barrier(&self, amount: i32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_add_barrier(self.ptr.as_ptr(), amount) } }
    pub fn increment(&self) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_increment(self.ptr.as_ptr()) } }
    pub fn increment_barrier(&self) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_increment_barrier(self.ptr.as_ptr()) } }
    pub fn decrement(&self) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_decrement(self.ptr.as_ptr()) } }
    pub fn decrement_barrier(&self) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_decrement_barrier(self.ptr.as_ptr()) } }
    pub fn or(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_or(self.ptr.as_ptr(), mask) } }
    pub fn or_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_or_barrier(self.ptr.as_ptr(), mask) } }
    pub fn or_orig(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_or_orig(self.ptr.as_ptr(), mask) } }
    pub fn or_orig_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_or_orig_barrier(self.ptr.as_ptr(), mask) } }
    pub fn and(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_and(self.ptr.as_ptr(), mask) } }
    pub fn and_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_and_barrier(self.ptr.as_ptr(), mask) } }
    pub fn and_orig(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_and_orig(self.ptr.as_ptr(), mask) } }
    pub fn and_orig_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_and_orig_barrier(self.ptr.as_ptr(), mask) } }
    pub fn xor(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_xor(self.ptr.as_ptr(), mask) } }
    pub fn xor_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_xor_barrier(self.ptr.as_ptr(), mask) } }
    pub fn xor_orig(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_xor_orig(self.ptr.as_ptr(), mask) } }
    pub fn xor_orig_barrier(&self, mask: u32) -> i32 { unsafe { ffi::apple_log_os_atomic_i32_xor_orig_barrier(self.ptr.as_ptr(), mask) } }
    pub fn compare_and_swap(&self, old_value: i32, new_value: i32) -> bool {
        unsafe { ffi::apple_log_os_atomic_i32_compare_and_swap(self.ptr.as_ptr(), old_value, new_value) }
    }
    pub fn compare_and_swap_barrier(&self, old_value: i32, new_value: i32) -> bool {
        unsafe {
            ffi::apple_log_os_atomic_i32_compare_and_swap_barrier(
                self.ptr.as_ptr(),
                old_value,
                new_value,
            )
        }
    }
}

impl Drop for OSAtomicI32 {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_atomic_i32_release(self.ptr.as_ptr()) };
    }
}

/// Safe wrapper around a 64-bit legacy `OSAtomic` value.
pub struct OSAtomicI64 {
    ptr: NonNull<c_void>,
}

impl OSAtomicI64 {
    #[must_use]
    pub fn new(value: i64) -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_atomic_i64_new(value) })
                .expect("Swift bridge never returns NULL for OSAtomicI64::new"),
        }
    }

    #[must_use]
    pub fn load(&self) -> i64 {
        unsafe { ffi::apple_log_os_atomic_i64_load(self.ptr.as_ptr()) }
    }

    pub fn store(&self, value: i64) {
        unsafe { ffi::apple_log_os_atomic_i64_store(self.ptr.as_ptr(), value) };
    }

    pub fn add(&self, amount: i64) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_add(self.ptr.as_ptr(), amount) } }
    pub fn add_barrier(&self, amount: i64) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_add_barrier(self.ptr.as_ptr(), amount) } }
    pub fn increment(&self) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_increment(self.ptr.as_ptr()) } }
    pub fn increment_barrier(&self) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_increment_barrier(self.ptr.as_ptr()) } }
    pub fn decrement(&self) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_decrement(self.ptr.as_ptr()) } }
    pub fn decrement_barrier(&self) -> i64 { unsafe { ffi::apple_log_os_atomic_i64_decrement_barrier(self.ptr.as_ptr()) } }
    pub fn compare_and_swap(&self, old_value: i64, new_value: i64) -> bool {
        unsafe { ffi::apple_log_os_atomic_i64_compare_and_swap(self.ptr.as_ptr(), old_value, new_value) }
    }
    pub fn compare_and_swap_barrier(&self, old_value: i64, new_value: i64) -> bool {
        unsafe {
            ffi::apple_log_os_atomic_i64_compare_and_swap_barrier(
                self.ptr.as_ptr(),
                old_value,
                new_value,
            )
        }
    }
}

impl Drop for OSAtomicI64 {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_atomic_i64_release(self.ptr.as_ptr()) };
    }
}

/// Queue backed by `OSAtomicEnqueue` / `OSAtomicDequeue`.
pub struct OSAtomicQueue {
    ptr: NonNull<c_void>,
}

impl OSAtomicQueue {
    /// Creates an empty queue.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge cannot allocate the queue.
    pub fn new() -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSAtomicQueue::new", |error_out| unsafe {
            ffi::apple_log_os_atomic_queue_new(error_out)
        })?;
        Ok(Self { ptr })
    }

    pub fn enqueue(&self, value: usize) {
        unsafe { ffi::apple_log_os_atomic_queue_enqueue(self.ptr.as_ptr(), value) };
    }

    #[must_use]
    pub fn dequeue(&self) -> Option<usize> {
        let mut value = 0_usize;
        unsafe { ffi::apple_log_os_atomic_queue_dequeue(self.ptr.as_ptr(), &mut value) }.then_some(value)
    }
}

impl Drop for OSAtomicQueue {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_atomic_queue_release(self.ptr.as_ptr()) };
    }
}

/// FIFO queue backed by `OSAtomicFifoEnqueue` / `OSAtomicFifoDequeue`.
pub struct OSAtomicFifoQueue {
    ptr: NonNull<c_void>,
}

impl OSAtomicFifoQueue {
    /// Creates an empty FIFO queue.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge cannot allocate the queue.
    pub fn new() -> Result<Self, LogError> {
        let ptr = bridge_ptr_result("OSAtomicFifoQueue::new", |error_out| unsafe {
            ffi::apple_log_os_atomic_fifo_queue_new(error_out)
        })?;
        Ok(Self { ptr })
    }

    pub fn enqueue(&self, value: usize) {
        unsafe { ffi::apple_log_os_atomic_fifo_queue_enqueue(self.ptr.as_ptr(), value) };
    }

    #[must_use]
    pub fn dequeue(&self) -> Option<usize> {
        let mut value = 0_usize;
        unsafe { ffi::apple_log_os_atomic_fifo_queue_dequeue(self.ptr.as_ptr(), &mut value) }
            .then_some(value)
    }
}

impl Drop for OSAtomicFifoQueue {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_atomic_fifo_queue_release(self.ptr.as_ptr()) };
    }
}
