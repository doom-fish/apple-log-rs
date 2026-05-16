use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        None,
    )?;
    if let Some(OSLogStoreEntry::Boundary(entry)) = entries.into_iter().find(|entry| matches!(entry, OSLogStoreEntry::Boundary(_))) {
        println!("boundary message={} is_boundary={}", entry.composed_message(), entry.is_boundary());
    } else {
        println!("no boundary entry found in current-process store");
    }
    Ok(())
}
