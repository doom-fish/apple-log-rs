//! API-surface coverage harness for `apple-log`.
//!
//! `os/log.h` is a tiny C header with macros that expand to internal
//! functions. We wrap them in our own C shim, so the "Apple symbols"
//! we need to track aren't the public-visible symbol set — they're the
//! conceptual API surface (create + emit at 5 levels).

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_header(rel: &str) -> String {
    let p = sdk_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

#[test]
fn os_log_macros_present_in_header() {
    let header = read_header("usr/include/os/log.h");
    // Verify every macro we shim exists in the SDK header.
    for sym in &[
        "os_log_create",
        "os_log_with_type",
        "os_log_debug",
        "os_log_info",
        "os_log_error",
        "os_log_fault",
        "OS_LOG_DEFAULT",
        "os_log_type_t",
    ] {
        assert!(header.contains(sym), "os/log.h missing {sym:?}");
    }
}

#[test]
fn os_log_type_constants_present() {
    let header = read_header("usr/include/os/log.h");
    for sym in &[
        "OS_LOG_TYPE_DEFAULT",
        "OS_LOG_TYPE_INFO",
        "OS_LOG_TYPE_DEBUG",
        "OS_LOG_TYPE_ERROR",
        "OS_LOG_TYPE_FAULT",
    ] {
        assert!(header.contains(sym), "os/log.h missing type constant {sym:?}");
    }
}

#[test]
fn rust_levels_match_apple_constants() {
    use apple_log::Level;
    // These must match `os_log_type_t` values from Apple's header.
    assert_eq!(Level::Default as i32, 0x00);
    assert_eq!(Level::Info as i32, 0x01);
    assert_eq!(Level::Debug as i32, 0x02);
    assert_eq!(Level::Error as i32, 0x10);
    assert_eq!(Level::Fault as i32, 0x11);
    let _ = BTreeSet::<i32>::new();
}
