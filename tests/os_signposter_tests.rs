use apple_log::prelude::*;

#[test]
fn os_signposter_smoke() {
    let signposter =
        OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("signposter");
    let id = signposter.make_signpost_id();
    signposter.emit_event("event", id, "payload");
    let interval = signposter.begin_animation_interval("anim", id, "begin");
    signposter.end_interval("anim", interval, "end");
    signposter.with_interval_signpost("wrapped", id, "around", || {});
    let _ = signposter.is_enabled();
    let _ = OSSignposter::default();
    let _ = OSSignposter::disabled();
}
