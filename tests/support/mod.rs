#![allow(dead_code)]

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use apple_log::prelude::*;

pub const SUBSYSTEM: &str = "fish.doom.apple-log.tests";

pub fn unique_category(label: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    format!("{label}-{}-{nanos}", std::process::id())
}

pub fn wait_for_entries(
    store: &OSLogStore,
    filter: &OSLogEntryFilter,
    expected: usize,
) -> Vec<OSLogStoreEntry> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let entries = store
            .get_entries(OSLogEnumeratorOptions::NONE, None, filter, 10_000)
            .expect("entries");
        if entries.len() >= expected || Instant::now() >= deadline {
            return entries;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn log_entries(entries: Vec<OSLogStoreEntry>) -> Vec<OSLogEntryLog> {
    entries
        .into_iter()
        .filter_map(|entry| match entry {
            OSLogStoreEntry::Log(log) => Some(log),
            _ => None,
        })
        .collect()
}

pub fn messages(entries: &[OSLogEntryLog]) -> Vec<String> {
    entries
        .iter()
        .map(OSLogEntryCommon::composed_message)
        .collect()
}
