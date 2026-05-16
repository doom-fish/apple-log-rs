# apple-log

Safe Rust bindings for Apple's [`os_log`](https://developer.apple.com/documentation/os/logging) on macOS — structured logging that integrates with Console.app and the `log` CLI.

> **Status:** experimental. v0.1 ships per-subsystem `Logger`, free-function logging via `OS_LOG_DEFAULT`, all 5 standard levels (Default / Info / Debug / Error / Fault). v0.2 adds `os_signpost` for performance tracing, `OSLogStore` for reading back logs, and `log` crate facade impl.

Pure C (with a tiny shim file built via `cc`) — **zero Swift bridge**.

## Quick start

```rust,no_run
use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free-function logging via OS_LOG_DEFAULT (no subsystem).
    log(Level::Info, "starting up");

    // Per-subsystem Logger (filter via `log stream --predicate ...`).
    let logger = Logger::new("fish.doom.myapp", "net")?;
    logger.info("opening port 8080");
    logger.debug("tcp socket fd=7");
    logger.error("connection refused");
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
- **Privacy-aware**: format specifiers (`%{public}s` vs `%{private}s`) control redaction.
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
- [x] `log()` free function via `OS_LOG_DEFAULT`
- [x] All 5 levels (Default / Info / Debug / Error / Fault)
- [x] `Send + Sync` `Logger`
- [ ] `os_signpost` for performance tracing
- [ ] `OSLogStore` to read back past logs from Rust
- [ ] `log` + `tracing` crate facade backends
- [ ] Format-string redaction control (`%{private}s` selector)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
