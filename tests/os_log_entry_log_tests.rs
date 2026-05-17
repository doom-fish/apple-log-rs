use std::time::Duration;

use apple_log::prelude::*;

#[test]
fn os_log_entry_log_accessors() {
    Logger::default().info("entry-log smoke");
    std::thread::sleep(Duration::from_millis(100));
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let entries = store
        .entries(
            OSLogEnumeratorOptions::REVERSE,
            Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
            None,
        )
        .expect("entries");
    if let Some(OSLogStoreEntry::Log(entry)) = entries
        .into_iter()
        .find(|entry| matches!(entry, OSLogStoreEntry::Log(_)))
    {
        let _ = entry.composed_message();
        let _ = entry.date();
        let _ = entry.store_category();
        let _ = entry.level();
        let _ = entry.activity_identifier();
        let _ = entry.process();
        let _ = entry.process_identifier();
        let _ = entry.sender();
        let _ = entry.thread_identifier();
        let _ = entry.category();
        let _ = entry.format_string();
        let _ = entry.subsystem();
        for component in entry.components() {
            let _ = component.format_substring();
            let _ = component.placeholder();
            let _ = component.argument_category();
            let _ = component.argument();
        }
    }
}
