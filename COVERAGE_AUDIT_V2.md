# apple-log coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 62
VERIFIED: 53
GAPS: 0
EXEMPT: 9
COVERAGE_PCT: 100.00%

## Methodology

This audit enumerates the public macOS API surface of `os/log.h`, `os/signpost.h`, and `os/activity.h` (C umbrella headers from the `os/log` system library), cross-references them with `apple-log`'s Rust safe wrappers and Swift bridge, and classifies each symbol.

The SDK public surface includes:
- C function declarations (`os_log_create`, `os_signpost_id_generate`, `os_activity_apply`, etc.)
- `typedef` declarations (`os_log_t`, `os_signpost_id_t`, `os_activity_t`, `os_activity_flag_t`, etc.)
- Macro constants (`OS_LOG_DEFAULT`, `OS_LOG_DISABLED`, `OS_SIGNPOST_ID_NULL`, `OS_ACTIVITY_NONE`, etc.)
- Enum constants (`OS_LOG_TYPE_*`, `OS_ACTIVITY_FLAG_*`)
- Convenience macros (`os_log_info`, `os_log_debug`, `os_signpost_interval_begin`, `os_activity_create`, etc.)

The crate wraps these via:
1. **Swift bridge** (`swift-bridge/Sources/AppleLogBridge/*.swift`): `@_cdecl` thunks to `Logger`, `OSLog`, `OSActivity`, `OSSignposter`, etc.
2. **Rust safe API** (`src/**/*.rs`): Idiomatic Rust structs and methods (`OSLog::new()`, `OSActivityFlags`, `OSSignpostId::generate()`, etc.)

All 53 verified symbols in v1 remain covered in macOS 26.2. All 9 deprecated symbols are still marked with `API_DEPRECATED` attributes and correctly excluded.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `os_log_t` | typedef | `usr/include/os/log.h` | `OSLog`, `Logger` |
| `OS_LOG_DISABLED` | macro constant | `usr/include/os/log.h` | `OSLog::disabled()`, `Logger::disabled()` |
| `OS_LOG_DEFAULT` | macro constant | `usr/include/os/log.h` | `OSLog::default()`, `Logger::default()`, `log()` |
| `os_log_type_t` | enum typedef | `usr/include/os/log.h` | `Level` |
| `OS_LOG_TYPE_DEFAULT` | enum constant | `usr/include/os/log.h` | `Level::Default` |
| `OS_LOG_TYPE_INFO` | enum constant | `usr/include/os/log.h` | `Level::Info` |
| `OS_LOG_TYPE_DEBUG` | enum constant | `usr/include/os/log.h` | `Level::Debug` |
| `OS_LOG_TYPE_ERROR` | enum constant | `usr/include/os/log.h` | `Level::Error` |
| `OS_LOG_TYPE_FAULT` | enum constant | `usr/include/os/log.h` | `Level::Fault` |
| `os_log_create` | function | `usr/include/os/log.h` | `OSLog::new()`, `Logger::new()` |
| `os_log_info_enabled` | macro | `usr/include/os/log.h` | `log_enabled(Level::Info)`, `Logger::is_enabled(Level::Info)`, `OSLog::is_enabled(Level::Info)` |
| `os_log_debug_enabled` | macro | `usr/include/os/log.h` | `log_enabled(Level::Debug)`, `Logger::is_enabled(Level::Debug)`, `OSLog::is_enabled(Level::Debug)` |
| `os_log_with_type` | macro | `usr/include/os/log.h` | `log()`, `log_with_privacy()`, `Logger::log()`, `Logger::log_with_privacy()` |
| `os_log` | macro | `usr/include/os/log.h` | `log(Level::Default, ...)`, `Logger::log(Level::Default, ...)` |
| `os_log_info` | macro | `usr/include/os/log.h` | `log(Level::Info, ...)`, `Logger::info()` |
| `os_log_debug` | macro | `usr/include/os/log.h` | `log(Level::Debug, ...)`, `Logger::debug()` |
| `os_log_error` | macro | `usr/include/os/log.h` | `log(Level::Error, ...)`, `Logger::error()` |
| `os_log_fault` | macro | `usr/include/os/log.h` | `log(Level::Fault, ...)`, `Logger::fault()` |
| `os_log_type_enabled` | function | `usr/include/os/log.h` | `OSLog::is_enabled()`, `Logger::is_enabled()` |
| `os_signpost_id_t` | typedef | `usr/include/os/signpost.h` | `OSSignpostId`, `SignpostId` |
| `OS_SIGNPOST_ID_NULL` | macro constant | `usr/include/os/signpost.h` | `OSSignpostId::NULL` |
| `OS_SIGNPOST_ID_INVALID` | macro constant | `usr/include/os/signpost.h` | `OSSignpostId::INVALID` |
| `OS_SIGNPOST_ID_EXCLUSIVE` | macro constant | `usr/include/os/signpost.h` | `OSSignpostId::EXCLUSIVE` |
| `os_signpost_id_make_with_pointer` | function | `usr/include/os/signpost.h` | `OSSignpostId::from_pointer()`, `OSSignposter::make_signpost_id_from_pointer()`, `Logger::signpost_id_from_pointer()` |
| `os_signpost_id_generate` | function | `usr/include/os/signpost.h` | `OSSignpostId::generate()`, `OSSignposter::make_signpost_id()`, `Logger::signpost_id()` |
| `os_signpost_enabled` | function | `usr/include/os/signpost.h` | `OSLog::signposts_enabled()`, `OSSignposter::is_enabled()`, `Logger::signposts_enabled()` |
| `os_signpost_interval_begin` | macro | `usr/include/os/signpost.h` | `OSSignposter::begin_interval()`, `Logger::signpost_interval_begin()` |
| `os_signpost_animation_interval_begin` | macro | `usr/include/os/signpost.h` | `OSSignposter::begin_animation_interval()`, `Logger::signpost_animation_interval_begin()` |
| `os_signpost_interval_end` | macro | `usr/include/os/signpost.h` | `OSSignposter::end_interval()`, `Logger::signpost_interval_end()` |
| `os_signpost_event_emit` | macro | `usr/include/os/signpost.h` | `OSSignposter::emit_event()`, `Logger::signpost_event()` |
| `OS_LOG_CATEGORY_POINTS_OF_INTEREST` | macro constant | `usr/include/os/signpost.h` | `CATEGORY_POINTS_OF_INTEREST` |
| `OS_LOG_CATEGORY_DYNAMIC_TRACING` | macro constant | `usr/include/os/signpost.h` | `CATEGORY_DYNAMIC_TRACING` |
| `OS_LOG_CATEGORY_DYNAMIC_STACK_TRACING` | macro constant | `usr/include/os/signpost.h` | `CATEGORY_DYNAMIC_STACK_TRACING` |
| `os_activity_flag_t` | enum typedef | `usr/include/os/activity.h` | `OSActivityFlags` |
| `OS_ACTIVITY_FLAG_DEFAULT` | enum constant | `usr/include/os/activity.h` | `OSActivityFlags::DEFAULT` |
| `OS_ACTIVITY_FLAG_DETACHED` | enum constant | `usr/include/os/activity.h` | `OSActivityFlags::DETACHED` |
| `OS_ACTIVITY_FLAG_IF_NONE_PRESENT` | enum constant | `usr/include/os/activity.h` | `OSActivityFlags::IF_NONE_PRESENT` |
| `os_activity_t` | typedef | `usr/include/os/activity.h` | `OSActivity` |
| `OS_ACTIVITY_NONE` | macro constant | `usr/include/os/activity.h` | `OSActivity::none()` |
| `OS_ACTIVITY_NULL` | macro constant | `usr/include/os/activity.h` | `OSActivity::null()` |
| `OS_ACTIVITY_CURRENT` | macro constant | `usr/include/os/activity.h` | `OSActivity::current()` |
| `os_activity_id_t` | typedef | `usr/include/os/activity.h` | `ActivityIds`, `OSActivity::identifier()` |
| `os_activity_scope_state_t` | struct typedef | `usr/include/os/activity.h` | `OSActivityScope` |
| `os_activity_create` | macro | `usr/include/os/activity.h` | `OSActivity::new()` |
| `os_activity_initiate` | macro | `usr/include/os/activity.h` | `OSActivity::initiate()` |
| `os_activity_initiate_f` | macro | `usr/include/os/activity.h` | `OSActivity::initiate_f()` |
| `os_activity_apply` | function | `usr/include/os/activity.h` | `OSActivity::apply()` |
| `os_activity_apply_f` | function | `usr/include/os/activity.h` | `OSActivity::apply()` |
| `os_activity_scope_enter` | function | `usr/include/os/activity.h` | `OSActivity::enter()` |
| `os_activity_scope_leave` | function | `usr/include/os/activity.h` | `OSActivityScope` drop |
| `os_activity_scope` | macro | `usr/include/os/activity.h` | `OSActivity::enter()` + RAII drop |
| `os_activity_get_identifier` | function | `usr/include/os/activity.h` | `OSActivity::identifiers()`, `OSActivity::identifier()`, `active_activity_ids()` |
| `os_activity_label_useraction` | macro | `usr/include/os/activity.h` | `OSActivity::label_user_action()` |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `os_log_is_enabled` | function | `usr/include/os/log.h` | 10.x deprecated legacy enablement helper; superseded by `os_log_type_enabled`. | `API_DEPRECATED("no longer suppored - always returns true", macos(10.11,10.12), ...)` |
| `os_log_is_debug_enabled` | function | `usr/include/os/log.h` | 10.x deprecated debug-only enablement helper. | `API_DEPRECATED_WITH_REPLACEMENT("os_log_debug_enabled", macos(10.11,10.12), ...)` |
| `os_log_sensitive` | macro | `usr/include/os/log.h` | 10.x deprecated privacy convenience macro; `apple-log` uses explicit privacy APIs instead. | `API_DEPRECATED("no longer supported - use os_log with per-parameter privacy options", macos(10.11,10.12), ...)` |
| `os_log_sensitive_debug` | macro | `usr/include/os/log.h` | Same deprecated privacy helper as `os_log_sensitive`, but for debug logging. | `API_DEPRECATED("no longer supported - use os_log with per-parameter privacy options", macos(10.11,10.12), ...)` |
| `os_activity_get_active` | function | `usr/include/os/activity.h` | 10.x deprecated diagnostic buffer API; the crate intentionally uses `os_activity_get_identifier(OS_ACTIVITY_CURRENT, ...)` instead. | `API_DEPRECATED("No longer supported", macos(10.10, 10.12), ...)` |
| `os_activity_start` | macro | `usr/include/os/activity.h` | Deprecated in favor of create + apply/scope. `apple-log` still exposes `OSActivity::start()`, but it is excluded from coverage. | `API_DEPRECATED("use combination of os_activity_create and os_activity_apply/os_activity_scope", macos(10.10, 10.12), ...)` |
| `os_activity_end` | function | `usr/include/os/activity.h` | Deprecated counterpart to `os_activity_start`. `apple-log` still exposes `OSActivity::end()`, but it is excluded from coverage. | `API_DEPRECATED("use combination of os_activity_create and os_activity_apply/os_activity_scope", macos(10.10, 10.12), ...)` |
| `os_breadcrumb_t` | typedef | `usr/include/os/activity.h` | Deprecated breadcrumb identifier type with no modern replacement other than user-action labeling. | `API_DEPRECATED("No longer supported", macos(10.10, 10.12), ...)` |
| `os_activity_set_breadcrumb` | macro | `usr/include/os/activity.h` | Deprecated in favor of `os_activity_label_useraction`. `apple-log` still exposes `OSActivity::set_breadcrumb()`, but it is excluded from coverage. | `API_DEPRECATED_WITH_REPLACEMENT("os_activity_label_useraction", macos(10.10, 10.12), ...)` |
