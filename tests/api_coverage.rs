//! API-surface coverage harness for `apple-log`.

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
fn os_log_symbols_present_in_header() {
    let header = read_header("usr/include/os/log.h");
    for sym in &[
        "OS_LOG_DEFAULT",
        "OS_LOG_DISABLED",
        "os_log_create",
        "os_log_with_type",
        "os_log_info",
        "os_log_debug",
        "os_log_error",
        "os_log_fault",
        "os_log_info_enabled",
        "os_log_debug_enabled",
        "os_log_type_t",
        "OS_LOG_TYPE_DEFAULT",
        "OS_LOG_TYPE_INFO",
        "OS_LOG_TYPE_DEBUG",
        "OS_LOG_TYPE_ERROR",
        "OS_LOG_TYPE_FAULT",
    ] {
        assert!(header.contains(sym), "os/log.h missing {sym:?}");
    }
}

#[test]
fn signpost_symbols_present_in_header() {
    let header = read_header("usr/include/os/signpost.h");
    for sym in &[
        "os_signpost_id_generate",
        "os_signpost_id_make_with_pointer",
        "os_signpost_enabled",
        "os_signpost_event_emit",
        "os_signpost_interval_begin",
        "os_signpost_animation_interval_begin",
        "os_signpost_interval_end",
        "OS_SIGNPOST_ID_NULL",
        "OS_SIGNPOST_ID_INVALID",
        "OS_SIGNPOST_ID_EXCLUSIVE",
        "OS_LOG_CATEGORY_POINTS_OF_INTEREST",
        "OS_LOG_CATEGORY_DYNAMIC_TRACING",
        "OS_LOG_CATEGORY_DYNAMIC_STACK_TRACING",
    ] {
        assert!(header.contains(sym), "os/signpost.h missing {sym:?}");
    }
}

#[test]
fn activity_symbols_present_in_header() {
    let header = read_header("usr/include/os/activity.h");
    for sym in &[
        "OS_ACTIVITY_CURRENT",
        "OS_ACTIVITY_NONE",
        "OS_ACTIVITY_FLAG_DEFAULT",
        "OS_ACTIVITY_FLAG_DETACHED",
        "OS_ACTIVITY_FLAG_IF_NONE_PRESENT",
        "os_activity_apply_f",
        "os_activity_scope_enter",
        "os_activity_scope_leave",
        "os_activity_get_identifier",
    ] {
        assert!(header.contains(sym), "os/activity.h missing {sym:?}");
    }
}

#[test]
fn os_atomic_symbols_present_in_header() {
    let header = read_header("usr/include/libkern/OSAtomicDeprecated.h");
    for sym in &[
        "OSAtomicAdd32",
        "OSAtomicCompareAndSwap32",
        "OSAtomicCompareAndSwap64",
        "OSAtomicCompareAndSwapPtr",
        "OSAtomicTestAndSet",
        "OSAtomicTestAndClear",
        "OSAtomicFifoEnqueue",
        "OSAtomicFifoDequeue",
    ] {
        assert!(header.contains(sym), "OSAtomicDeprecated.h missing {sym:?}");
    }
    let queue_header = read_header("usr/include/libkern/OSAtomicQueue.h");
    for sym in &["OSAtomicEnqueue", "OSAtomicDequeue", "OSQueueHead"] {
        assert!(queue_header.contains(sym), "OSAtomicQueue.h missing {sym:?}");
    }
}

#[test]
fn rust_constants_match_apple_values() {
    use apple_log::{
        Level, SignpostId, CATEGORY_DYNAMIC_STACK_TRACING, CATEGORY_DYNAMIC_TRACING,
        CATEGORY_POINTS_OF_INTEREST,
    };

    assert_eq!(Level::Default as i32, 0x00);
    assert_eq!(Level::Info as i32, 0x01);
    assert_eq!(Level::Debug as i32, 0x02);
    assert_eq!(Level::Error as i32, 0x10);
    assert_eq!(Level::Fault as i32, 0x11);

    assert_eq!(SignpostId::NULL.as_u64(), 0);
    assert_eq!(SignpostId::INVALID.as_u64(), u64::MAX);
    assert_eq!(SignpostId::EXCLUSIVE.as_u64(), 0xEEEE_B0B5_B2B2_EEEE);

    assert_eq!(CATEGORY_POINTS_OF_INTEREST, "PointsOfInterest");
    assert_eq!(CATEGORY_DYNAMIC_TRACING, "DynamicTracing");
    assert_eq!(CATEGORY_DYNAMIC_STACK_TRACING, "DynamicStackTracing");

    let _ = BTreeSet::<u64>::new();
}
