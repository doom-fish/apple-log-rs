use std::time::Duration;

use apple_log::prelude::*;

#[test]
fn os_log_entry_signpost_accessors() {
    let signposter =
        OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("signposter");
    let id = signposter.make_signpost_id();
    let interval = signposter.begin_interval("signpost-test", id, "begin");
    signposter.end_interval("signpost-test", interval, "end");
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let entries = store
        .entries(
            OSLogEnumeratorOptions::REVERSE,
            Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
            None,
        )
        .expect("entries");
    if let Some(OSLogStoreEntry::Signpost(entry)) = entries
        .into_iter()
        .find(|entry| matches!(entry, OSLogStoreEntry::Signpost(_)))
    {
        let _ = entry.composed_message();
        let _ = entry.signpost_identifier();
        let _ = entry.signpost_name();
        let _ = entry.signpost_type();
        let _ = entry.category();
        let _ = entry.subsystem();
    }
}
