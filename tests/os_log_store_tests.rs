use std::time::Duration;

use apple_log::prelude::*;

#[test]
fn os_log_store_smoke() {
    Logger::default().info("store smoke");
    std::thread::sleep(Duration::from_millis(100));
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let position = store.position_time_interval_since_end(Duration::from_secs(5));
    let entries = store
        .entries(OSLogEnumeratorOptions::REVERSE, Some(&position), None)
        .expect("entries");
    assert!(
        !entries.is_empty(),
        "expected at least one current-process log entry"
    );
    let _ = OSLogStore::local().expect("local store");
}
