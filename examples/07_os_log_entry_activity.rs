use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let activity = OSActivity::new(
        "example-activity",
        Some(&OSActivity::current()),
        OSActivityFlags::DEFAULT,
    )?;
    activity.apply(|| {
        Logger::default().info("inside example activity");
    });
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        None,
    )?;
    if let Some(OSLogStoreEntry::Activity(entry)) = entries
        .into_iter()
        .find(|entry| matches!(entry, OSLogStoreEntry::Activity(_)))
    {
        println!(
            "activity process={} parent={}",
            entry.process(),
            entry.parent_activity_identifier()
        );
    } else {
        println!("no activity entry found in current-process store");
    }
    Ok(())
}
