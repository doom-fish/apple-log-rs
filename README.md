# apple-log

Safe Rust bindings for Apple's unified logging stack on macOS.

`apple-log` v0.6 adds a Swift bridge on top of the C `os` APIs and the Swift `os` / `OSLog` modules, covering:

- `Logger`
- `OSLog`
- `OSLogStore`
- `OSLogEntryLog`
- `OSLogEntrySignpost`
- `OSLogEntryBoundary`
- `OSLogEntryActivity`
- `OSSignpostID`
- `OSSignposter`
- `OSActivity`

> **Platform:** macOS 12+ (the bridge uses Swift `Logger` and `OSSignposter`).

## Quick start

```rust,no_run
use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.myapp", "network")?;
    logger.info("user@example.com signed in");
    logger.log_with_privacy(Level::Info, "cache warmed", Privacy::Public);

    let signposter = OSSignposter::new("fish.doom.myapp", CATEGORY_POINTS_OF_INTEREST)?;
    let signpost_id = signposter.make_signpost_id();
    let interval = signposter.begin_interval(c"startup", signpost_id, "begin startup");
    signposter.end_interval(interval, "end startup");

    let activity = OSActivity::new(
        c"index cache",
        Some(&OSActivity::current()),
        OSActivityFlags::DEFAULT,
    )?;
    activity.apply(|| Logger::default().info("inside activity"));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        &OSLogEntryFilter::default()
            .subsystem("fish.doom.myapp")
            .min_level(Level::Error),
        1_000,
    )?;
    println!("recent errors: {}", entries.len());

    Ok(())
}
```

## Privacy

Messages are logged as private unless you opt in. `Logger::log`, the `trace`..`fault`
convenience calls, the `log` free function, signpost messages and the raw
`apple_log_emit`/`apple_log_emit_default` shim functions all redact the message as
`<private>` in the unified log (unless private data logging is enabled on the machine).
Use `log_with_privacy(.., Privacy::Public)` or the `*_with_privacy` signpost methods
for text that is safe to store in clear.

## Signpost and activity names

Signpost names, activity descriptions, user-action labels and breadcrumbs are
`&'static CStr`. Pass string literals such as `c"startup"`: the unified log records
only the location of these strings and decodes them from the binary, so text built at
run time cannot be decoded. Each signpost call emits exactly one signpost, and
`OSSignpostInterval` remembers the name its interval began with.

## Reading the log store

`OSLogStore::get_entries` takes an `OSLogEntryFilter` (subsystem, category, minimum
level, date range) and a `max_entries` bound. Filter values are passed to the
predicate as arguments, never as predicate syntax. `get_entries_with_predicate`
accepts a raw `NSPredicate` format string for other queries; invalid predicates
return an error instead of aborting the process.

`OSLogStoreScope::CurrentProcessIdentifier` reads the calling process's own entries.
`OSLogStore::local()` and `OSLogStoreScope::System` read the whole system log and can
fail with a permission error for users who are not administrators.

## Areas and modules

- `apple_log::logger::Logger` and compatibility free functions in `apple_log::log`
- `apple_log::os_log::OSLog`
- `apple_log::os_log_store::{OSLogStore, OSLogEntryFilter, OSLogPosition, OSLogStoreEntry}`
- `apple_log::os_log_entry_*` typed entry wrappers
- `apple_log::os_signpost_id::OSSignpostId`
- `apple_log::os_signposter::OSSignposter`
- `apple_log::os_activity::OSActivity`

## Raw C FFI

The crate keeps the low-level C shim behind the `raw-ffi` feature. The feature is enabled by default for backwards compatibility.

```toml
[dependencies]
apple-log = { version = "0.5", default-features = false }
```

Enable `raw-ffi` when you want direct access to the wrapped C symbols under `apple_log::ffi`.

## Async activity instrumentation

Enable the `async` feature to wrap any executor-agnostic future in an `OSActivity` scope that is re-entered on every poll.

```toml
[dependencies]
apple-log = { version = "0.6", features = ["async"] }
```

```rust,no_run
use apple_log::{OSActivity, OSActivityFlags};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let bytes = OSActivity::new(
    c"download asset",
    Some(&OSActivity::current()),
    OSActivityFlags::DEFAULT,
)?
.instrument_future(async { 42_usize })
.await?;

assert_eq!(bytes, 42);
# Ok(())
# }
```

## Examples

The crate ships one numbered example per logical area in `examples/01_logger.rs` through `examples/10_os_activity.rs`.

## Coverage

See [COVERAGE.md](COVERAGE.md) for the SDK audit and implementation matrix.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
