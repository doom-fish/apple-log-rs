//! Demonstrate public/private payloads plus current activity ids.
//!
//! Run: `cargo run --example 03_privacy_and_activity`

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.apple-log", "privacy")?;
    logger.log_with_privacy(Level::Info, "visible line", Privacy::Public);
    logger.log_with_privacy(Level::Info, "secret line", Privacy::Private);
    log_with_privacy(
        Level::Debug,
        "default logger private line",
        Privacy::Private,
    );

    let ids = active_activity_ids();
    println!("default enabled(info): {}", log_enabled(Level::Info));
    println!(
        "disabled enabled(info): {}",
        Logger::disabled().is_enabled(Level::Info)
    );
    println!("current activity id: {}", ids.current);
    println!("parent activity id: {:?}", ids.parent);

    Ok(())
}
