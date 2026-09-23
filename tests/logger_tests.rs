mod support;

use apple_log::prelude::*;
use support::{log_entries, messages, unique_category, wait_for_entries, SUBSYSTEM};

#[test]
fn logger_smoke() {
    let logger = Logger::new("fish.doom.apple-log", "logger-tests").expect("logger");
    logger.trace("trace");
    logger.debug("debug");
    logger.info("info");
    logger.notice("notice");
    logger.warning("warning");
    logger.error("error");
    logger.critical("critical");
    logger.fault("fault");
    logger.log_with_privacy(Level::Info, "private", Privacy::Private);
    let _ = logger.is_enabled(Level::Info);
    let _ = Logger::default();
    let _ = Logger::disabled();
    assert_eq!(
        log_enabled(Level::Fault),
        Logger::default().is_enabled(Level::Fault)
    );
}

fn private_data_is_revealed(
    logger: &Logger,
    store: &OSLogStore,
    filter: &OSLogEntryFilter,
) -> bool {
    logger.log_with_privacy(Level::Error, "control-private-token", Privacy::Private);
    logger.log_with_privacy(Level::Error, "control-public-token", Privacy::Public);
    let entries = log_entries(wait_for_entries(store, filter, 2));
    let messages = messages(&entries);
    assert!(
        messages
            .iter()
            .any(|message| message == "control-public-token"),
        "explicitly public messages stay readable: {messages:?}"
    );
    messages
        .iter()
        .any(|message| message.contains("control-private-token"))
}

#[test]
fn convenience_logging_redacts_dynamic_strings_by_default() {
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let control = Logger::new(SUBSYSTEM, &unique_category("privacy-control")).expect("logger");
    let control_filter = OSLogEntryFilter::default().subsystem(SUBSYSTEM);
    if private_data_is_revealed(&control, &store, &control_filter) {
        eprintln!("private log data is revealed on this machine; skipping redaction checks");
        return;
    }

    let category = unique_category("privacy");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    logger.notice("notice-private-token");
    logger.warning("warning-private-token");
    logger.error("error-private-token");
    logger.critical("critical-private-token");
    logger.fault("fault-private-token");
    logger.log(Level::Default, "log-private-token");
    logger.log_with_privacy(Level::Error, "explicit-public-token", Privacy::Public);

    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    let entries = log_entries(wait_for_entries(&store, &filter, 7));
    let messages = messages(&entries);
    assert_eq!(messages.len(), 7, "{messages:?}");
    assert!(messages
        .iter()
        .any(|message| message == "explicit-public-token"));
    assert!(
        messages
            .iter()
            .all(|message| !message.contains("private-token")),
        "dynamic strings must be private unless opted in: {messages:?}"
    );
}

#[test]
fn free_functions_redact_dynamic_strings_by_default() {
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let control = Logger::new(SUBSYSTEM, &unique_category("free-control")).expect("logger");
    let control_filter = OSLogEntryFilter::default().subsystem(SUBSYSTEM);
    if private_data_is_revealed(&control, &store, &control_filter) {
        eprintln!("private log data is revealed on this machine; skipping redaction checks");
        return;
    }

    let token = unique_category("free-private-token");
    let public = unique_category("free-public-token");
    log(Level::Error, &token);
    log_with_privacy(Level::Error, &public, Privacy::Public);

    let everything = OSLogEntryFilter::default().min_level(Level::Error);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let messages = loop {
        let messages = messages(&log_entries(
            store
                .get_entries(OSLogEnumeratorOptions::NONE, None, &everything, 100_000)
                .expect("entries"),
        ));
        if messages.contains(&public) || std::time::Instant::now() >= deadline {
            break messages;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    };
    assert!(
        messages.contains(&public),
        "public free-function log is readable"
    );
    assert!(
        !messages.iter().any(|message| message.contains(&token)),
        "free-function logs are private by default"
    );
}

#[cfg(feature = "raw-ffi")]
#[test]
fn raw_shim_emit_defaults_to_private() {
    use std::ffi::CString;

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let control = Logger::new(SUBSYSTEM, &unique_category("raw-control")).expect("logger");
    let control_filter = OSLogEntryFilter::default().subsystem(SUBSYSTEM);
    if private_data_is_revealed(&control, &store, &control_filter) {
        eprintln!("private log data is revealed on this machine; skipping redaction checks");
        return;
    }

    let category = unique_category("raw");
    let subsystem = CString::new(SUBSYSTEM).expect("subsystem");
    let category_c = CString::new(category.as_str()).expect("category");
    unsafe {
        let log = apple_log::ffi::apple_log_create(subsystem.as_ptr(), category_c.as_ptr());
        apple_log::ffi::apple_log_emit(log, 0x10, c"raw-private-token".as_ptr());
        apple_log::ffi::apple_log_emit_privacy(log, 0x10, c"raw-public-token".as_ptr(), true);
        apple_log::ffi::apple_log_release(log);
    }

    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    let messages = messages(&log_entries(wait_for_entries(&store, &filter, 2)));
    assert_eq!(messages.len(), 2, "{messages:?}");
    assert!(messages.iter().any(|message| message == "raw-public-token"));
    assert!(!messages
        .iter()
        .any(|message| message.contains("raw-private-token")));
}
