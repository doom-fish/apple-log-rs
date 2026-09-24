# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - Unreleased

### Security

- Dynamic strings passed to `Logger` convenience calls, the `log` free function, signposts and the C shim's `apple_log_emit` / `apple_log_emit_default` were stored in clear text in the unified log; they are now private unless explicitly made public (see Changed).
- `OSLogStore` no longer passes caller text to `NSPredicate(format:)`, which read nonexistent varargs and aborted the process on malformed input (see Changed).

### Fixed

- Signposts use the caller's name as the signpost name instead of `"rust"`, and `OSSignposter::begin_interval` / `end_interval` no longer emit an extra event signpost each.
- Activity descriptions, user-action labels and breadcrumbs are constant strings, so the unified log can decode them.
- `get_entries` stops enumerating at `max_entries` instead of materializing every matching entry.
- Entry dates never panic: invalid values map to `UNIX_EPOCH`, and dates before 1970 are supported.
- The OSLogStore bridge no longer uses trapping integer conversions on framework values.
- The `log`, `log_with_privacy` and `log_enabled` free functions no longer allocate a Swift `Logger` on every call.

### Changed

- **Breaking:** `Logger::log`, `trace` through `fault`, the `log` free function, signpost messages and the C shim's `apple_log_emit` / `apple_log_emit_default` log their message as private; pass `Privacy::Public` through `log_with_privacy`, a `*_with_privacy` method or the `*_privacy` shim functions to opt in.
- **Breaking:** `OSLogStore::get_entries` takes an `OSLogEntryFilter` and a `max_entries` bound instead of `Option<&str>`; raw predicate formats go through `get_entries_with_predicate`.
- **Breaking:** the `raw-ffi` feature is no longer enabled by default. The raw shim is `unsafe` and trusts callers to pass constant strings where the SDK requires them, so it is now an explicit opt-in: `features = ["raw-ffi"]`.
- **Breaking:** the compatibility aliases `OSLogStore::entries` (use `get_entries`) and `SignpostId` (use `OSSignpostId`) and the `apple_log::log` compatibility module are removed. `log`, `log_with_privacy` and `log_enabled` now live in `apple_log::logger`, and `active_activity_id`, `active_activity_ids` and `ActivityIds` in `apple_log::os_activity`; they are still re-exported at the crate root and in the prelude.
- **Breaking:** signpost names are `&'static CStr` in `OSSignposter::emit_event`, `begin_interval`, `begin_animation_interval`, `with_interval_signpost` and the `Logger::signpost_*` helpers; `OSSignposter::end_interval` takes the name from the `OSSignpostInterval`.
- **Breaking:** `OSActivity::new`, `start`, `initiate`, `initiate_f`, `label_user_action` and `set_breadcrumb` take `&'static CStr`; `initiate` and `initiate_f` return `()`.
- **Breaking:** the OSAtomic wrappers (`OSAtomicI32`, `OSAtomicI64`, `OSAtomicQueue`, `OSAtomicFifoQueue`) and their raw FFI are removed. They wrapped deprecated APIs, were `!Send`/`!Sync` and freed queue nodes unsafely; use `std::sync::atomic`.
- `OSActivity` is `Send + Sync`, so `ActivityFuture` is `Send` when its inner future is.
- `rust-version` is now 1.82.
- `build.rs` no longer runs `swiftlint`.

### Added

- `OSLogEntryFilter` (subsystem, category, minimum level, date range) and `OSLogStore::get_entries_with_predicate`, which parses raw predicate formats inside an Objective-C `@try/@catch` and returns an error for invalid input.
- `OSSignpostInterval::name`, the `OSSignposter::*_with_privacy` methods, `Logger::signpost_event_with_privacy`, `ffi::signpost_kind` and the raw `apple_signpost_emit`.

### Removed

- The unused duplicate C shim `src/c-shim/apple_log_shim.c`.

## [0.6.0] - 2026-05-20

### Added

- `async` feature plus `async_api::ActivityFuture`, an executor-agnostic wrapper that enters an `OSActivity` around every future poll.
- `OSActivity::instrument_future` plus refreshed OSActivity async instrumentation docs and examples.

### Notes

- Phase 32 completeness + async sweep.

## [0.5.2] - 2025-01-09

### Changed

- Added SAFETY comments to all unsafe blocks for improved code clarity and auditing.

## [0.5.1] - 2026-05-16

### Added

- `OSActivity::null()` for the explicit `OS_ACTIVITY_NULL` sentinel.
- `OSActivity::{initiate, initiate_f}` convenience helpers for `os_activity_initiate` and `os_activity_initiate_f`.

### Changed

- `COVERAGE.md` and `COVERAGE_AUDIT.md` now mark the remaining `os/activity.h` gaps as implemented.
- The OSActivity example and smoke tests now exercise the null sentinel plus initiate helpers.

## [0.5.0] - 2026-05-16

### Added

- SwiftPM-based bridge build that wraps both the C `os` APIs and the Swift `os` / `OSLog` modules.
- Safe wrappers for `OSLog`, `OSLogStore`, `OSLogEntryLog`, `OSLogEntrySignpost`, `OSLogEntryBoundary`, `OSLogEntryActivity`, `OSSignpostId`, `OSSignposter`, `OSActivity`, and `OSAtomic`.
- One numbered example and one smoke test for each logical area.
- `COVERAGE.md` auditing the public SDK surface used by the crate.

### Changed

- `Logger` now uses the Swift `Logger` bridge while keeping the existing compatibility API intact.
- Raw C FFI is now isolated behind the `raw-ffi` feature (still enabled by default for backwards compatibility).
- Build orchestration now follows the multi-file Swift bridge pattern used across the Apple SDK crates.

## [0.4.0] - 2026-05-16

### Added

- Public/private redaction control via `Privacy`, `Logger::log_with_privacy`, and `log_with_privacy`.
- Borrowed `Logger::default()` and `Logger::disabled()` handles for `OS_LOG_DEFAULT` / `OS_LOG_DISABLED`.
- Signpost helpers for `os_signpost_id_make_with_pointer`, animation intervals, signpost id constants, and signpost category constants.
- `ActivityIds` plus `active_activity_ids()` for current + parent activity-id introspection.
- Smoke examples for signposts and privacy/activity usage.

### Changed

- README status/roadmap now matches the shipped `os_log` / `os_signpost` / `os_activity` surface.
- API coverage tests now verify signpost and activity header symbols in addition to base logging macros.

## [0.1.0] - Initial release

### Added

- `Logger::new(subsystem, category)` — wraps `os_log_create`.
- `Logger::{log, info, debug, error, fault}` convenience API.
- `log(Level, msg)` free function via `OS_LOG_DEFAULT`.
- `Level` enum matching Apple's 5 standard levels.
- `Logger::is_enabled(level)` via `os_log_type_enabled`.
- Signpost helpers: generated signpost IDs, enablement check, events, and begin/end intervals.
- `active_activity_id()` current-activity helper.
- Example `01_basic_log` and basic header coverage tests.
