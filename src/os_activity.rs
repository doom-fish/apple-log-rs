#![allow(clippy::missing_panics_doc, clippy::use_self)]

use core::ffi::c_void;
use core::ops::{BitOr, BitOrAssign};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::ptr::NonNull;

use crate::bridge_support::{bridge_ptr_result, c_string_arg, sanitized_c_string};
use crate::error::LogError;
use crate::ffi;
#[cfg(feature = "async")]
use std::future::Future;

/// Current activity id plus its optional parent id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivityIds {
    pub current: u64,
    pub parent: Option<u64>,
}

/// Flags for activity creation and deprecated activity start APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OSActivityFlags(u32);

impl OSActivityFlags {
    pub const DEFAULT: Self = Self(0);
    pub const DETACHED: Self = Self(0x1);
    pub const IF_NONE_PRESENT: Self = Self(0x2);

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl BitOr for OSActivityFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for OSActivityFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Scoped activity guard returned by `OSActivity::enter`.
pub struct OSActivityScope {
    ptr: NonNull<c_void>,
}

impl Drop for OSActivityScope {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_activity_scope_leave(self.ptr.as_ptr()) };
    }
}

/// Safe wrapper around `os_activity_t`.
pub struct OSActivity {
    ptr: NonNull<c_void>,
}

struct ApplyContext<F> {
    closure: Option<F>,
    panic: Option<Box<dyn std::any::Any + Send>>,
}

struct ApplyWithContext<C, F> {
    context: *mut C,
    function: Option<F>,
    panic: Option<Box<dyn std::any::Any + Send>>,
}

unsafe extern "C" fn apply_trampoline<F>(context: *mut c_void)
where
    F: FnOnce(),
{
    let context = &mut *context.cast::<ApplyContext<F>>();
    let closure = context
        .closure
        .take()
        .expect("OSActivity apply trampoline called at most once");
    if let Err(panic) = catch_unwind(AssertUnwindSafe(closure)) {
        context.panic = Some(panic);
    }
}

unsafe extern "C" fn apply_with_context_trampoline<C, F>(context: *mut c_void)
where
    F: FnOnce(&mut C),
{
    let context = &mut *context.cast::<ApplyWithContext<C, F>>();
    let function = context
        .function
        .take()
        .expect("OSActivity apply_with_context trampoline called at most once");
    let value = &mut *context.context;
    if let Err(panic) = catch_unwind(AssertUnwindSafe(|| function(value))) {
        context.panic = Some(panic);
    }
}

impl OSActivity {
    /// Creates a new activity.
    ///
    /// Pass `None` to use `OS_ACTIVITY_CURRENT`, or `Some(&OSActivity::null())`
    /// to pass the explicit `OS_ACTIVITY_NULL` sentinel.
    ///
    /// # Errors
    ///
    /// Returns an error if the description contains a NUL byte or the bridge fails.
    pub fn new(
        description: &str,
        parent: Option<&OSActivity>,
        flags: OSActivityFlags,
    ) -> Result<Self, LogError> {
        let description = c_string_arg("description", description)?;
        let ptr = bridge_ptr_result("OSActivity::new", |error_out| unsafe {
            ffi::apple_log_os_activity_create(
                description.as_ptr(),
                parent.map_or(std::ptr::null_mut(), OSActivity::as_ptr),
                flags.bits(),
                error_out,
            )
        })?;
        Ok(Self { ptr })
    }

    /// Creates a deprecated started activity.
    ///
    /// # Errors
    ///
    /// Returns an error if the description contains a NUL byte or the bridge fails.
    pub fn start(description: &str, flags: OSActivityFlags) -> Result<Self, LogError> {
        let description = c_string_arg("description", description)?;
        let ptr = bridge_ptr_result("OSActivity::start", |error_out| unsafe {
            ffi::apple_log_os_activity_start(description.as_ptr(), flags.bits(), error_out)
        })?;
        Ok(Self { ptr })
    }

    /// Synchronously initiates an activity around a closure.
    ///
    /// # Errors
    ///
    /// Returns an error if the description contains a NUL byte.
    pub fn initiate<F>(
        description: &str,
        flags: OSActivityFlags,
        closure: F,
    ) -> Result<(), LogError>
    where
        F: FnOnce(),
    {
        let description = c_string_arg("description", description)?;
        let mut context = ApplyContext {
            closure: Some(closure),
            panic: None,
        };
        unsafe {
            ffi::apple_log_os_activity_initiate(
                description.as_ptr(),
                flags.bits(),
                std::ptr::addr_of_mut!(context).cast(),
                Some(apply_trampoline::<F>),
            );
        }
        if let Some(panic) = context.panic {
            resume_unwind(panic);
        }
        Ok(())
    }

    /// Synchronously initiates an activity and passes a mutable context value to the callback.
    ///
    /// # Errors
    ///
    /// Returns an error if the description contains a NUL byte.
    pub fn initiate_f<C, F>(
        description: &str,
        flags: OSActivityFlags,
        context: &mut C,
        function: F,
    ) -> Result<(), LogError>
    where
        F: FnOnce(&mut C),
    {
        let description = c_string_arg("description", description)?;
        let mut bridge_context = ApplyWithContext {
            context: std::ptr::from_mut(context),
            function: Some(function),
            panic: None,
        };
        unsafe {
            ffi::apple_log_os_activity_initiate_f(
                description.as_ptr(),
                flags.bits(),
                std::ptr::addr_of_mut!(bridge_context).cast(),
                Some(apply_with_context_trampoline::<C, F>),
            );
        }
        if let Some(panic) = bridge_context.panic {
            resume_unwind(panic);
        }
        Ok(())
    }

    #[must_use]
    pub fn current() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_activity_current() })
                .expect("Swift bridge never returns NULL for OSActivity.current"),
        }
    }

    #[must_use]
    pub fn none() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_activity_none() })
                .expect("Swift bridge never returns NULL for OSActivity.none"),
        }
    }

    #[must_use]
    pub fn null() -> Self {
        Self {
            ptr: NonNull::new(unsafe { ffi::apple_log_os_activity_null() })
                .expect("Swift bridge never returns NULL for OSActivity.null"),
        }
    }

    #[must_use]
    pub fn identifiers(&self) -> ActivityIds {
        let mut parent = 0_u64;
        let current =
            unsafe { ffi::apple_log_os_activity_get_identifier(self.ptr.as_ptr(), &mut parent) };
        ActivityIds {
            current,
            parent: (parent != 0).then_some(parent),
        }
    }

    #[must_use]
    pub fn identifier(&self) -> u64 {
        self.identifiers().current
    }

    pub fn apply<F>(&self, closure: F)
    where
        F: FnOnce(),
    {
        let mut context = ApplyContext {
            closure: Some(closure),
            panic: None,
        };
        unsafe {
            ffi::apple_log_os_activity_apply(
                self.ptr.as_ptr(),
                std::ptr::addr_of_mut!(context).cast(),
                Some(apply_trampoline::<F>),
            );
        }
        if let Some(panic) = context.panic {
            resume_unwind(panic);
        }
    }

    /// Wrap a future so this activity is entered around every poll.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub const fn instrument_future<F>(self, future: F) -> crate::async_api::ActivityFuture<F>
    where
        F: Future,
    {
        crate::async_api::ActivityFuture::new(self, future)
    }

    /// Enters the activity for the current scope.
    ///
    /// # Errors
    ///
    /// Returns an error if the bridge cannot allocate scope state.
    pub fn enter(&self) -> Result<OSActivityScope, LogError> {
        let ptr =
            NonNull::new(unsafe { ffi::apple_log_os_activity_scope_enter(self.ptr.as_ptr()) })
                .ok_or_else(|| LogError::bridge("OSActivity::enter returned NULL"))?;
        Ok(OSActivityScope { ptr })
    }

    pub fn end(&self) {
        unsafe { ffi::apple_log_os_activity_end(self.ptr.as_ptr()) };
    }

    pub fn label_user_action(label: &str) {
        let label = sanitized_c_string(label);
        unsafe { ffi::apple_log_os_activity_label_useraction(label.as_ptr()) };
    }

    pub fn set_breadcrumb(name: &str) {
        let name = sanitized_c_string(name);
        unsafe { ffi::apple_log_os_activity_set_breadcrumb(name.as_ptr()) };
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr()
    }
}

impl Drop for OSActivity {
    fn drop(&mut self) {
        unsafe { ffi::apple_log_os_activity_release(self.ptr.as_ptr()) };
    }
}

/// Returns the current thread's active activity id.
#[must_use]
pub fn active_activity_id() -> u64 {
    active_activity_ids().current
}

/// Returns the current thread's active activity id and parent activity id.
#[must_use]
pub fn active_activity_ids() -> ActivityIds {
    let mut parent = 0_u64;
    let current = unsafe { ffi::apple_log_os_activity_get_active_identifiers(&mut parent) };
    ActivityIds {
        current,
        parent: (parent != 0).then_some(parent),
    }
}
