# apple-log

Safe Rust bindings for Apple's [`os_log`](https://developer.apple.com/documentation/os/logging) on macOS — structured logging that integrates with Console.app and the `log` CLI.

> **Status:** actively developed. v0.4 ships per-subsystem `Logger`, free-function logging via `OS_LOG_DEFAULT`, all 5 standard levels, public/private redaction control, default/disabled log handles, signpost helpers, and current activity-id introspection.

Pure C (with a tiny shim file built via `cc`) — **zero Swift bridge**.

## Quick start

```rust,no_run
use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log(Level::Info, "starting up");
    log_with_privacy(Level::Debug, "token=secret", Privacy::Private);

    let logger = Logger::new("fish.doom.myapp", CATEGORY_POINTS_OF_INTEREST)?;
    logger.info("opening port 8080");
    logger.debug("tcp socket fd=7");

    let span = logger.signpost_id();
    logger.signpost_interval_begin(span, "startup");
    logger.signpost_interval_end(span, "startup");

    let activity = active_activity_ids();
    println!("current activity id: {}", activity.current);
    Ok(())
}
```

Then in another terminal:
```sh
log stream --predicate 'subsystem == "fish.doom.myapp"'
log show --info --debug --predicate 'subsystem == "fish.doom.myapp"' --last 1m
```

Or open **Console.app** and filter by `subsystem`.

## Why `os_log` over `log`/`tracing`?

- **System-integrated**: visible in Console.app, `log` CLI, system-wide log archives — even after your binary exits.
- **Persistent**: error+ entries survive log rotation.
- **Privacy-aware**: `Privacy::Public` vs `Privacy::Private` controls redaction.
- **No I/O cost**: the kernel buffers log records; expensive-to-format messages are skipped when no subscriber is active.
- **Free for the user**: integrates with existing macOS log infrastructure (sysdiagnose, MDM exports, etc.).

You can still wire `log`/`tracing` on top — both crates support pluggable backends.

```text
your-app ──► apple-log ──► /private/var/db/diagnostics
                                  │
                                  ├──► Console.app
                                  ├──► `log` CLI
                                  └──► sysdiagnose archives
```

## Roadmap

- [x] `Logger::new(subsystem, category)`
- [x] `Logger::{info, debug, error, fault}` + `log(level, msg)` shortcut
- [x] Public/private redaction control
- [x] `Logger::default()` / `Logger::disabled()` handles
- [x] Signpost ids, events, intervals, and animation intervals
- [x] Signpost category constants (`PointsOfInterest`, `DynamicTracing`, `DynamicStackTracing`)
- [x] Current activity id + parent-id introspection
- [ ] Activity creation / scope helpers for named activities
- [ ] `log` + `tracing` crate facade backends

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
