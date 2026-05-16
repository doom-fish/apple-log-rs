use apple_log::prelude::*;

#[test]
fn os_log_smoke() {
    let log = OSLog::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("log");
    let _ = log.is_enabled(Level::Info);
    let _ = log.signposts_enabled();
    assert_eq!(log.subsystem().as_deref(), Some("fish.doom.apple-log"));
    assert_eq!(log.category().as_deref(), Some(CATEGORY_POINTS_OF_INTEREST));
    let _ = OSLog::default();
    let _ = OSLog::disabled();
}
