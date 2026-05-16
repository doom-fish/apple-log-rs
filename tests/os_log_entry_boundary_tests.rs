use std::time::Duration;

use apple_log::prelude::*;

#[test]
fn os_log_entry_boundary_accessors() {
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let entries = store
        .entries(
            OSLogEnumeratorOptions::REVERSE,
            Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
            None,
        )
        .expect("entries");
    if let Some(OSLogStoreEntry::Boundary(entry)) = entries.into_iter().find(|entry| matches!(entry, OSLogStoreEntry::Boundary(_))) {
        let _ = entry.composed_message();
        let _ = entry.date();
        let _ = entry.store_category();
        let _ = entry.is_boundary();
    }
}
