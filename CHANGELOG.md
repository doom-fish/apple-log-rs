# Changelog

## [0.1.0] - Initial release

### Added

- `Logger::new(subsystem, category)` opens an `os_log_t` handle.
- `Logger::{info, debug, error, fault, log(level, msg)}` emit messages
  at all 5 standard `os_log_type_t` levels.
- `log(level, msg)` free function emits via `OS_LOG_DEFAULT`.
- `Level` enum mirroring Apple's `os_log_type_t`.
- `LogError`: `InvalidArgument`, `CreateFailed`.
- Tiny C shim (`src/c-shim/apple_log_shim.c`, built via `cc`) wraps the
  `os_log_with_type` macro since Rust can't invoke C macros directly.
- `Logger` is `Send + Sync` — `os_log_t` is thread-safe per Apple docs.

### Verification

- `cargo run --example 01_basic_log` emits 6 messages visible via
  `log stream --predicate 'subsystem == "fish.doom.apple-log"'`.
- 3 API-coverage tests verify the os_log macros + type constants exist
  in the SDK header and our `Level` enum matches Apple's numeric values.
