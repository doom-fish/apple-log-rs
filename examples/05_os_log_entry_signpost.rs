use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signposter = OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST)?;
    let id = signposter.make_signpost_id();
    let interval = signposter.begin_interval("startup", id, "begin");
    std::thread::sleep(Duration::from_millis(5));
    signposter.end_interval("startup", interval, "end");
    signposter.emit_event("milestone", id, "event");
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        None,
    )?;
    if let Some(OSLogStoreEntry::Signpost(entry)) = entries
        .into_iter()
        .find(|entry| matches!(entry, OSLogStoreEntry::Signpost(_)))
    {
        println!(
            "name={} type={:?} id={}",
            entry.signpost_name(),
            entry.signpost_type(),
            entry.signpost_identifier()
        );
    } else {
        println!("no signpost entry found in current-process store");
    }
    Ok(())
}
