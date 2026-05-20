//! Executor-agnostic async instrumentation helpers for [`OSActivity`](crate::OSActivity).
//!
//! Enable this module with the `async` Cargo feature:
//!
//! ```toml
//! [dependencies]
//! apple-log = { version = "0.6", features = ["async"] }
//! ```
//!
//! [`ActivityFuture`] wraps any `Future` and enters an `OSActivity` around each
//! `poll`. This keeps activity scoping aligned with executor wakeups without
//! holding a scope guard across suspension points.
//!
//! # Example
//!
//! ```rust,no_run
//! use apple_log::{OSActivity, OSActivityFlags};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let bytes = OSActivity::new(
//!     "download asset",
//!     Some(&OSActivity::current()),
//!     OSActivityFlags::DEFAULT,
//! )?
//! .instrument_future(async { 42_usize })
//! .await?;
//!
//! assert_eq!(bytes, 42);
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]
#![allow(clippy::module_name_repetitions)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{LogError, OSActivity};

/// Future wrapper that re-enters an [`OSActivity`] every time the executor polls it.
#[must_use = "futures do nothing unless awaited or polled"]
pub struct ActivityFuture<F> {
    activity: OSActivity,
    future: F,
}

impl<F> ActivityFuture<F> {
    /// Wrap `future` so each poll executes inside `activity`.
    pub const fn new(activity: OSActivity, future: F) -> Self {
        Self { activity, future }
    }

    /// Borrow the activity that will be entered around each poll.
    #[must_use]
    pub const fn activity(&self) -> &OSActivity {
        &self.activity
    }
}

impl<F> ActivityFuture<F>
where
    F: Future,
{
    /// Unwrap the wrapper and return the owned activity plus inner future.
    #[must_use]
    pub fn into_parts(self) -> (OSActivity, F) {
        (self.activity, self.future)
    }
}

impl<F> Future for ActivityFuture<F>
where
    F: Future,
{
    type Output = Result<F::Output, LogError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: projecting `future` is sound because the outer `ActivityFuture`
        // is pinned for the duration of this call and we never move `future`
        // after obtaining the pinned mutable reference.
        let this = unsafe { self.get_unchecked_mut() };
        let _scope = match this.activity.enter() {
            Ok(scope) => scope,
            Err(error) => return Poll::Ready(Err(error)),
        };

        // SAFETY: see comment above for the pinned projection rationale.
        unsafe { Pin::new_unchecked(&mut this.future) }
            .poll(cx)
            .map(Ok)
    }
}

/// Wrap `future` so each poll executes inside `activity`.
pub const fn instrument_future<F>(activity: OSActivity, future: F) -> ActivityFuture<F> {
    ActivityFuture::new(activity, future)
}
