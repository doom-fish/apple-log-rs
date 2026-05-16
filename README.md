# apple-log

Safe Rust bindings for Apple's unified logging stack on macOS.

`apple-log` v0.5 adds a Swift bridge on top of the C `os` APIs and the Swift `os` / `OSLog` modules, covering:

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
- `OSAtomic`

> **Platform:** macOS 12+ (the bridge uses Swift `Logger` and `OSSignposter`).

## Quick start

```rust,no_run
use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.myapp", "network")?;
    logger.info("booting service");
    logger.log_with_privacy(Level::Info, "token=secret", Privacy::Private);

    let signposter = OSSignposter::new("fish.doom.myapp", CATEGORY_POINTS_OF_INTEREST)?;
    let signpost_id = signposter.make_signpost_id();
    let interval = signposter.begin_interval("startup", signpost_id, "begin startup");
    signposter.end_interval("startup", interval, "end startup");

    let activity = OSActivity::new(
        "index cache",
        Some(&OSActivity::current()),
        OSActivityFlags::DEFAULT,
    )?;
    activity.apply(|| Logger::default().info("inside activity"));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        None,
    )?;
    println!("recent entries: {}", entries.len());

    Ok(())
}
```

## Areas and modules

- `apple_log::logger::Logger` and compatibility free functions in `apple_log::log`
- `apple_log::os_log::OSLog`
- `apple_log::os_log_store::{OSLogStore, OSLogPosition, OSLogStoreEntry}`
- `apple_log::os_log_entry_*` typed entry wrappers
- `apple_log::os_signpost_id::OSSignpostId`
- `apple_log::os_signposter::OSSignposter`
- `apple_log::os_activity::OSActivity`
- `apple_log::os_atomic::{OSAtomicI32, OSAtomicI64, OSAtomicQueue, OSAtomicFifoQueue}`

## Raw C FFI

The crate keeps the low-level C shim behind the `raw-ffi` feature. The feature is enabled by default for backwards compatibility.

```toml
[dependencies]
apple-log = { version = "0.5", default-features = false }
```

Enable `raw-ffi` when you want direct access to the wrapped C symbols under `apple_log::ffi`.

## Examples

The crate ships one numbered example per logical area in `examples/01_logger.rs` through `examples/11_os_atomic.rs`.

## Coverage

See [COVERAGE.md](COVERAGE.md) for the SDK audit and implementation matrix.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
