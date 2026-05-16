//! Emit a few signposts.
//!
//! Run: `cargo run --example 02_signposts`

use std::time::Duration;

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST)?;
    let generated = logger.signpost_id();
    let pointer = logger.signpost_id_from_pointer(std::ptr::addr_of!(logger).cast::<u8>());

    println!("signposts enabled: {}", logger.signposts_enabled());
    println!("generated id: {}", generated.as_u64());
    println!("pointer id:   {}", pointer.as_u64());

    logger.signpost_event(generated, "startup", "basic signpost event");
    logger.signpost_animation_interval_begin(SignpostId::EXCLUSIVE, "frame");
    std::thread::sleep(Duration::from_millis(10));
    logger.signpost_interval_end(SignpostId::EXCLUSIVE, "frame");

    Ok(())
}
