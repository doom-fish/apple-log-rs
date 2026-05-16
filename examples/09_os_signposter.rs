use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signposter = OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST)?;
    let id = signposter.make_signpost_id();
    signposter.emit_event("phase", id, "event");
    let interval = signposter.begin_animation_interval("animation", id, "begin animation");
    std::thread::sleep(Duration::from_millis(5));
    signposter.end_interval("animation", interval, "end animation");
    signposter.with_interval_signpost("wrapped", id, "around", || std::thread::sleep(Duration::from_millis(1)));
    println!("enabled={} id={}", signposter.is_enabled(), id.as_u64());
    Ok(())
}
