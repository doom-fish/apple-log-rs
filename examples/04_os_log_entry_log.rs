use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.apple-log", "entry-log")?;
    logger.info("entry-log probe");
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let entries = store.entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&store.position_time_interval_since_end(Duration::from_secs(5))),
        None,
    )?;
    if let Some(OSLogStoreEntry::Log(entry)) = entries.into_iter().find(|entry| matches!(entry, OSLogStoreEntry::Log(_))) {
        println!("message={} level={:?}", entry.composed_message(), entry.level());
        println!("subsystem={} category={} process={}", entry.subsystem(), entry.category(), entry.process());
    } else {
        println!("no log entry found in current-process store");
    }
    Ok(())
}
