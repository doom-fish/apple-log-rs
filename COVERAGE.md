# apple-log coverage audit

Reference inputs:

- `usr/include/os/log.h`
- `usr/include/os/signpost.h`
- `usr/include/os/activity.h`
- Swift symbol graphs for modules `os` and `OSLog`

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

## Logger (`os` Swift module)

| API | Status | Notes |
| --- | --- | --- |
| `Logger.init(subsystem:category:)` | ✅ | `Logger::new` |
| `Logger.init()` | ✅ | `Logger::default` |
| `Logger.init(_:)` | ✅ | `Logger::from_os_log` |
| `Logger.log(_:)` | ✅ | `Logger::log(Level::Default, ...)` |
| `Logger.log(level:_:)` | ✅ | `Logger::log` |
| `Logger.trace(_:)` | ✅ | `Logger::trace` |
| `Logger.debug(_:)` | ✅ | `Logger::debug` |
| `Logger.info(_:)` | ✅ | `Logger::info` |
| `Logger.notice(_:)` | ✅ | `Logger::notice` |
| `Logger.warning(_:)` | ✅ | `Logger::warning` |
| `Logger.error(_:)` | ✅ | `Logger::error` |
| `Logger.critical(_:)` | ✅ | `Logger::critical` |
| `Logger.fault(_:)` | ✅ | `Logger::fault` |

String payloads are logged as private; `Logger::log_with_privacy` with `Privacy::Public` opts into clear text.

## OSLog (`os` Swift module + `os/log.h`)

| API | Status | Notes |
| --- | --- | --- |
| `OSLog.default` | ✅ | `OSLog::default` |
| `OSLog.disabled` | ✅ | `OSLog::disabled` |
| `OSLog.init(subsystem:category:)` | ✅ | `OSLog::new` |
| `OSLog.Category.pointsOfInterest` | ✅ | `CATEGORY_POINTS_OF_INTEREST` |
| `OSLog.Category.dynamicTracing` | ✅ | `CATEGORY_DYNAMIC_TRACING` |
| `OSLog.Category.dynamicStackTracing` | ✅ | `CATEGORY_DYNAMIC_STACK_TRACING` |
| `OSLog.signpostsEnabled` | ✅ | `OSLog::signposts_enabled` |
| `OSLog.isEnabled(type:)` | ✅ | `OSLog::is_enabled` |
| `os_log_create` | ✅ | safe `OSLog::new`, raw `ffi::apple_log_create` |
| `OS_LOG_DEFAULT` / `OS_LOG_DISABLED` | ✅ | safe `OSLog::{default,disabled}`, raw `ffi::*` |
| `os_log_with_type` / `os_log_*` macros | ✅ | safe `Logger` / compatibility API, raw shim symbols |
| `os_log_type_enabled` | ✅ | safe `Logger::is_enabled` / `OSLog::is_enabled`, raw `ffi::apple_log_type_enabled` |
| `OSLogType.default/info/debug/error/fault` | ✅ | `Level` |

## OSLogStore and entry snapshots (`OSLog` Swift module)

| API | Status | Notes |
| --- | --- | --- |
| `OSLogStore.local()` | ✅ | `OSLogStore::local` |
| `OSLogStore.init(scope:)` | ✅ | `OSLogStore::new` |
| `OSLogStore.init(url:)` / `init(URL:)` | ✅ | `OSLogStore::from_url` |
| `OSLogStore.position(date:)` | ✅ | `OSLogStore::position_at` |
| `OSLogStore.position(timeIntervalSinceEnd:)` | ✅ | `OSLogStore::position_time_interval_since_end` |
| `OSLogStore.position(timeIntervalSinceLatestBoot:)` | ✅ | `OSLogStore::position_time_interval_since_latest_boot` |
| `OSLogStore.Scope.system` | ✅ | `OSLogStoreScope::System` |
| `OSLogStore.Scope.currentProcessIdentifier` | ✅ | `OSLogStoreScope::CurrentProcessIdentifier` |
| `OSLogStore.getEntries(with:at:matching:)` | ✅ | `OSLogStore::get_entries` / `entries` with an `OSLogEntryFilter` (subsystem, category, minimum level, date range) built from `%K`/`%@` arguments and a `max_entries` bound; `OSLogStore::get_entries_with_predicate` for raw predicate formats, parsed inside `@try/@catch` so invalid input returns an error |
| `OSLogEnumerator.Options.reverse` | ✅ | `OSLogEnumeratorOptions::REVERSE` |
| `OSLogPosition` | ✅ | `OSLogPosition` wrapper |
| `OSLogEntry.date` | ✅ | `OSLogEntryCommon::date` |
| `OSLogEntry.composedMessage` | ✅ | `OSLogEntryCommon::composed_message` |
| `OSLogEntry.storeCategory` | ✅ | `OSLogEntryCommon::store_category` |
| `OSLogEntry.StoreCategory.*` | ✅ | `OSLogStoreCategory` |
| `OSLogEntryFromProcess.activityIdentifier` | ✅ | `OSLogEntryFromProcess::activity_identifier` |
| `OSLogEntryFromProcess.process` | ✅ | `OSLogEntryFromProcess::process` |
| `OSLogEntryFromProcess.processIdentifier` | ✅ | `OSLogEntryFromProcess::process_identifier` |
| `OSLogEntryFromProcess.sender` | ✅ | `OSLogEntryFromProcess::sender` |
| `OSLogEntryFromProcess.threadIdentifier` | ✅ | `OSLogEntryFromProcess::thread_identifier` |
| `OSLogEntryWithPayload.category` | ✅ | `OSLogEntryWithPayload::category` |
| `OSLogEntryWithPayload.components` | ✅ | `OSLogEntryWithPayload::components` |
| `OSLogEntryWithPayload.formatString` | ✅ | `OSLogEntryWithPayload::format_string` |
| `OSLogEntryWithPayload.subsystem` | ✅ | `OSLogEntryWithPayload::subsystem` |
| `OSLogEntryLog.level` | ✅ | `OSLogEntryLog::level` |
| `OSLogEntryLog.Level.*` | ✅ | `OSLogEntryLogLevel` |
| `OSLogEntrySignpost.signpostIdentifier` | ✅ | `OSLogEntrySignpost::signpost_identifier` |
| `OSLogEntrySignpost.signpostName` | ✅ | `OSLogEntrySignpost::signpost_name` |
| `OSLogEntrySignpost.signpostType` | ✅ | `OSLogEntrySignpost::signpost_type` |
| `OSLogEntrySignpost.SignpostType.*` | ✅ | `OSLogEntrySignpostType` |
| `OSLogEntryBoundary` | ✅ | `OSLogEntryBoundary` wrapper |
| `OSLogEntryActivity.parentActivityIdentifier` | ✅ | `OSLogEntryActivity::parent_activity_identifier` |
| `OSLogMessageComponent.formatSubstring` | ✅ | `OSLogMessageComponent::format_substring` |
| `OSLogMessageComponent.placeholder` | ✅ | `OSLogMessageComponent::placeholder` |
| `OSLogMessageComponent.argumentCategory` | ✅ | `OSLogMessageArgumentCategory` |
| `OSLogMessageComponent.argumentDataValue` | ✅ | `OSLogMessageArgument::Data` |
| `OSLogMessageComponent.argumentDoubleValue` | ✅ | `OSLogMessageArgument::Double` |
| `OSLogMessageComponent.argumentInt64Value` | ✅ | `OSLogMessageArgument::Signed` |
| `OSLogMessageComponent.argumentStringValue` | ✅ | `OSLogMessageArgument::String` |
| `OSLogMessageComponent.argumentUInt64Value` | ✅ | `OSLogMessageArgument::Unsigned` |
| `OSLogMessageComponent.Argument` | ✅ | `OSLogMessageArgument` |
| `OSLogMessageComponent.ArgumentCategory.*` | ✅ | `OSLogMessageArgumentCategory` |

## OSSignpostID and OSSignposter (`os` Swift module + `os/signpost.h`)

| API | Status | Notes |
| --- | --- | --- |
| `OSSignpostID.null` / `invalid` / `exclusive` | ✅ | `OSSignpostId::{NULL,INVALID,EXCLUSIVE}` |
| `OSSignpostID.init(log:)` | ✅ | `OSSignpostId::generate` |
| `OSSignpostID.init(log:object:)` | ✅ | `OSSignpostId::from_pointer` (pointer-based equivalent) |
| `OSSignpostID.init(_:)` | ✅ | `OSSignpostId::from_u64` |
| `OSSignpostID.rawValue` | ✅ | `OSSignpostId::as_u64` |
| `OSSignposter.init(subsystem:category:)` | ✅ | `OSSignposter::new` |
| `OSSignposter.init()` | ✅ | `OSSignposter::default` |
| `OSSignposter.init(logHandle:)` | ✅ | `OSSignposter::from_os_log` |
| `OSSignposter.init(logger:)` | ✅ | `OSSignposter::from_logger` |
| `OSSignposter.disabled` | ✅ | `OSSignposter::disabled` |
| `OSSignposter.isEnabled` | ✅ | `OSSignposter::is_enabled` |
| `OSSignposter.makeSignpostID()` | ✅ | `OSSignposter::make_signpost_id` |
| `OSSignposter.makeSignpostID(from:)` | ✅ | `OSSignposter::make_signpost_id_from_pointer` |
| `OSSignposter.emitEvent` | ✅ | `OSSignposter::emit_event` / `emit_event_with_privacy` |
| `OSSignposter.beginInterval` | ✅ | `OSSignposter::begin_interval` / `begin_interval_with_privacy` |
| `OSSignposter.beginAnimationInterval` | ✅ | `OSSignposter::begin_animation_interval` / `begin_animation_interval_with_privacy` |
| `OSSignposter.endInterval` | ✅ | `OSSignposter::end_interval` / `end_interval_with_privacy` |
| `OSSignposter.withIntervalSignpost` | ✅ | `OSSignposter::with_interval_signpost` |
| `os_signpost_id_generate` | ✅ | safe + raw |
| `os_signpost_id_make_with_pointer` | ✅ | safe + raw |
| `os_signpost_enabled` | ✅ | safe + raw |
| `os_signpost_event_emit` | ✅ | safe + raw |
| `os_signpost_interval_begin` | ✅ | safe + raw |
| `os_signpost_animation_interval_begin` | ✅ | safe + raw |
| `os_signpost_interval_end` | ✅ | safe + raw |
| `OS_LOG_CATEGORY_*` | ✅ | category constants |

Signpost names are `&'static CStr` literals, as the SDK requires string constants; messages are private unless a `*_with_privacy` method passes `Privacy::Public`.

## OSActivity (`os/activity.h`)

| API | Status | Notes |
| --- | --- | --- |
| `OS_ACTIVITY_CURRENT` / `OS_ACTIVITY_NONE` / `OS_ACTIVITY_NULL` | ✅ | `OSActivity::{current,none,null}` |
| `OS_ACTIVITY_FLAG_DEFAULT` | ✅ | `OSActivityFlags::DEFAULT` |
| `OS_ACTIVITY_FLAG_DETACHED` | ✅ | `OSActivityFlags::DETACHED` |
| `OS_ACTIVITY_FLAG_IF_NONE_PRESENT` | ✅ | `OSActivityFlags::IF_NONE_PRESENT` |
| `os_activity_create` | ✅ | `OSActivity::new` |
| `os_activity_initiate` / `os_activity_initiate_f` | ✅ | `OSActivity::{initiate,initiate_f}` |
| `os_activity_apply` / `os_activity_apply_f` | ✅ | `OSActivity::apply` |
| `os_activity_scope_enter` / `os_activity_scope_leave` | ✅ | `OSActivity::enter` + `OSActivityScope` |
| `os_activity_get_identifier` | ✅ | `OSActivity::identifiers` / `active_activity_ids` |
| `os_activity_label_useraction` | ✅ | `OSActivity::label_user_action` |
| `os_activity_start` / `os_activity_end` | ✅ | `OSActivity::start` / `OSActivity::end` |
| `os_activity_set_breadcrumb` | ✅ | `OSActivity::set_breadcrumb` |
| `os_activity_get_active` | ⏭️ skipped | Deprecated diagnostic-only buffer API; `active_activity_ids` covers supported identifier access. |

## OSAtomic (`libkern/OSAtomic*.h`)

Removed in 0.7.0. The OSAtomic functions are deprecated since macOS 10.12 and the FIFO queue is no longer supported since macOS 11. The former wrappers were `!Send`/`!Sync`, loaded and stored with plain accesses, and freed queue nodes that concurrent dequeuers could still read. Use `std::sync::atomic` instead.

## Intentionally skipped Swift compile-time logging helpers

| API | Status | Notes |
| --- | --- | --- |
| `os_log(_:dso:log:_:_:)` Swift global overloads | ⏭️ skipped | Compile-time interpolation wrappers; the crate exposes `Logger` and compatibility functions instead. |
| `os_signpost(_:dso:log:name:signpostID:...)` Swift global overloads | ⏭️ skipped | Compile-time interpolation wrappers; covered by `OSSignposter` and raw signpost shim. |
| `OSLogMessage`, `OSLogInterpolation`, formatting/privacy helper enums | ⏭️ skipped | Compiler-assisted string interpolation types are not stable FFI surface; the bridge accepts plain Rust strings. |
