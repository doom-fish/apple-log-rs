# apple-log coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 53
VERIFIED: 50
GAPS: 3
EXEMPT: 9
COVERAGE_PCT: 94.34%

Scope notes:
- Counted documented macro entrypoints and macro constants alongside typedefs and exported functions because `os/log.h`, `os/signpost.h`, and `os/activity.h` expose much of their public surface that way.
- Treated the safe Rust message-based wrappers as coverage for the corresponding C macros even though the SDK's compile-time format-string machinery is not exposed 1:1.
- Excluded underscore-prefixed implementation helpers (`_os_log_impl`, `_os_signpost_emit_with_name_impl`, `_os_activity_*`) and the `Signpost Internals` enum surface to avoid double-counting compiler/runtime plumbing.
- Kept 10.x-deprecated APIs in the audit, but listed them as EXEMPT even when `apple-log` still exposes a wrapper.

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
| `OS_ACTIVITY_CURRENT` | macro constant | `usr/include/os/activity.h` | `OSActivity::current()` |
| `os_activity_id_t` | typedef | `usr/include/os/activity.h` | `ActivityIds`, `OSActivity::identifier()` |
| `os_activity_scope_state_t` | struct typedef | `usr/include/os/activity.h` | `OSActivityScope` |
| `os_activity_create` | macro | `usr/include/os/activity.h` | `OSActivity::new()` |
| `os_activity_apply` | function | `usr/include/os/activity.h` | `OSActivity::apply()` |
| `os_activity_apply_f` | function | `usr/include/os/activity.h` | `OSActivity::apply()` |
| `os_activity_scope_enter` | function | `usr/include/os/activity.h` | `OSActivity::enter()` |
| `os_activity_scope_leave` | function | `usr/include/os/activity.h` | `OSActivityScope` drop |
| `os_activity_scope` | macro | `usr/include/os/activity.h` | `OSActivity::enter()` + RAII drop |
| `os_activity_get_identifier` | function | `usr/include/os/activity.h` | `OSActivity::identifiers()`, `OSActivity::identifier()`, `active_activity_ids()` |
| `os_activity_label_useraction` | macro | `usr/include/os/activity.h` | `OSActivity::label_user_action()` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `OS_ACTIVITY_NULL` | macro constant | `usr/include/os/activity.h` | No explicit Rust sentinel for the null activity. `OSActivity::new(..., None, ...)` resolves to `OS_ACTIVITY_CURRENT`, not `OS_ACTIVITY_NULL`. |
| `os_activity_initiate` | macro | `usr/include/os/activity.h` | Can be composed with `OSActivity::new(...)?` + `OSActivity::apply(...)`, but the crate has no one-call equivalent for the documented convenience macro. |
| `os_activity_initiate_f` | macro | `usr/include/os/activity.h` | Same functional gap as `os_activity_initiate`: no dedicated initiate-and-run helper for the function-pointer form. |

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
