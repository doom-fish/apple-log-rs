mod support;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use apple_log::prelude::*;
use support::{log_entries, unique_category, wait_for_entries, SUBSYSTEM};

#[test]
fn os_log_store_smoke() {
    Logger::default().info("store smoke");
    std::thread::sleep(Duration::from_millis(100));
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let position = store.position_time_interval_since_end(Duration::from_secs(5));
    let entries = store
        .entries(
            OSLogEnumeratorOptions::REVERSE,
            Some(&position),
            &OSLogEntryFilter::default(),
            1_000,
        )
        .expect("entries");
    assert!(
        !entries.is_empty(),
        "expected at least one current-process log entry"
    );
    let _ = OSLogStore::local().expect("local store");
}

#[test]
fn structured_filter_matches_subsystem_category_and_level() {
    let category = unique_category("levels");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    let other = Logger::new(SUBSYSTEM, &unique_category("other")).expect("logger");
    logger.log_with_privacy(Level::Default, "filter default", Privacy::Public);
    logger.log_with_privacy(Level::Error, "filter error", Privacy::Public);
    logger.log_with_privacy(Level::Fault, "filter fault", Privacy::Public);
    other.log_with_privacy(Level::Fault, "other fault", Privacy::Public);

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let everything = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    let all = log_entries(wait_for_entries(&store, &everything, 3));
    assert_eq!(all.len(), 3, "all three levels are visible to the filter");

    let severe = everything.min_level(Level::Error);
    let entries = log_entries(wait_for_entries(&store, &severe, 2));
    let mut messages: Vec<String> = entries
        .iter()
        .map(OSLogEntryCommon::composed_message)
        .collect();
    messages.sort();
    assert_eq!(messages, ["filter error", "filter fault"]);
    for entry in &entries {
        assert_eq!(entry.subsystem(), SUBSYSTEM);
        assert_eq!(entry.category(), category);
        assert!(matches!(
            entry.level(),
            OSLogEntryLogLevel::Error | OSLogEntryLogLevel::Fault
        ));
    }
}

#[test]
fn filter_values_are_arguments_not_predicate_syntax() {
    let category = unique_category("quote\" OR subsystem != \"x");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    logger.log_with_privacy(Level::Error, "quoted category", Privacy::Public);

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    let entries = log_entries(wait_for_entries(&store, &filter, 1));
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].category(), category);

    let injected = OSLogEntryFilter::default().subsystem("x\" OR subsystem != \"x");
    let entries = store
        .get_entries(OSLogEnumeratorOptions::NONE, None, &injected, 100)
        .expect("entries");
    assert!(
        entries.is_empty(),
        "a quoted value must not widen the query"
    );
}

#[test]
fn date_range_filter_excludes_entries_outside_the_range() {
    let category = unique_category("dates");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    logger.log_with_privacy(Level::Error, "dated entry", Privacy::Public);

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    assert_eq!(wait_for_entries(&store, &filter, 1).len(), 1);

    let long_ago = filter
        .clone()
        .until(UNIX_EPOCH + Duration::from_secs(86_400));
    let entries = store
        .get_entries(OSLogEnumeratorOptions::NONE, None, &long_ago, 100)
        .expect("entries");
    assert!(entries.is_empty());

    let recent = filter.since(SystemTime::now() - Duration::from_secs(600));
    let entries = store
        .get_entries(OSLogEnumeratorOptions::NONE, None, &recent, 100)
        .expect("entries");
    assert_eq!(entries.len(), 1);
}

#[test]
fn max_entries_bounds_the_result() {
    let category = unique_category("bounded");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    for index in 0..5 {
        logger.log_with_privacy(Level::Error, &format!("bounded {index}"), Privacy::Public);
    }

    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    assert_eq!(wait_for_entries(&store, &filter, 5).len(), 5);
    let entries = store
        .get_entries(OSLogEnumeratorOptions::NONE, None, &filter, 2)
        .expect("entries");
    assert_eq!(entries.len(), 2);
    let entries = store
        .get_entries(OSLogEnumeratorOptions::NONE, None, &filter, 0)
        .expect("entries");
    assert!(entries.is_empty());
}

#[test]
fn raw_predicates_report_errors_instead_of_aborting() {
    let store = OSLogStore::new(OSLogStoreScope::CurrentProcessIdentifier).expect("store");
    for predicate in [
        "subsystem ==== ",
        "subsystem == %@",
        "%K == 'x'",
        "nosuchkey == 1",
    ] {
        let result =
            store.get_entries_with_predicate(OSLogEnumeratorOptions::NONE, None, predicate, 10);
        assert!(
            matches!(result, Err(LogError::BridgeError(_))),
            "{predicate:?} should be rejected"
        );
    }
    assert!(matches!(
        store.get_entries_with_predicate(OSLogEnumeratorOptions::NONE, None, "a\0b", 10),
        Err(LogError::InvalidArgument(_))
    ));

    let category = unique_category("raw");
    let logger = Logger::new(SUBSYSTEM, &category).expect("logger");
    logger.log_with_privacy(Level::Error, "raw predicate", Privacy::Public);
    let filter = OSLogEntryFilter::default()
        .subsystem(SUBSYSTEM)
        .category(category.as_str());
    assert_eq!(wait_for_entries(&store, &filter, 1).len(), 1);
    let entries = store
        .get_entries_with_predicate(
            OSLogEnumeratorOptions::NONE,
            None,
            &format!("subsystem == '{SUBSYSTEM}' AND category == '{category}'"),
            10,
        )
        .expect("valid raw predicate");
    assert_eq!(entries.len(), 1);
}
