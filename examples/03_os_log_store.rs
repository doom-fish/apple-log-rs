use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.apple-log", "store")?;
    logger.info("store probe");
    std::thread::sleep(Duration::from_millis(100));

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier)?;
    let position = store.position_time_interval_since_end(Duration::from_secs(5));
    let entries = store.get_entries(
        OSLogEnumeratorOptions::REVERSE,
        Some(&position),
        &OSLogEntryFilter::default(),
        1_000,
    )?;
    println!("entries={}", entries.len());
    Ok(())
}
