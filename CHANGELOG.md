# Changelog

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
