use apple_log::prelude::*;

#[test]
fn os_signpost_id_smoke() {
    let log = OSLog::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("log");
    let generated = OSSignpostId::generate(&log);
    let from_pointer = OSSignpostId::from_pointer(&log, std::ptr::addr_of!(log));
    assert!(OSSignpostId::NULL.is_null());
    assert!(OSSignpostId::INVALID.is_invalid());
    assert_ne!(generated.as_u64(), 0);
    assert_ne!(from_pointer.as_u64(), 0);
}
