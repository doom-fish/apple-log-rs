use std::time::Duration;

use apple_log::prelude::*;

#[test]
fn os_log_entry_activity_accessors() {
    let activity = OSActivity::new("activity-entry-test", Some(&OSActivity::current()), OSActivityFlags::DEFAULT).expect("activity");
    activity.apply(|| Logger::default().info("inside activity test"));
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let entries = store
        .entries(
            OSLogEnumeratorOptions::REVERSE,
            Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
            None,
        )
        .expect("entries");
    if let Some(OSLogStoreEntry::Activity(entry)) = entries.into_iter().find(|entry| matches!(entry, OSLogStoreEntry::Activity(_))) {
        let _ = entry.composed_message();
        let _ = entry.parent_activity_identifier();
        let _ = entry.process();
    }
}
