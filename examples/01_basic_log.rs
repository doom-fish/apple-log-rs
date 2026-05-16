//! Emit a log line at every level. Then run in another terminal:
//!
//!   log stream --predicate 'subsystem == "fish.doom.apple-log"'
//!
//! to see the messages flow.
//!
//! Run: `cargo run --example 01_basic_log`

use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Emitting messages — watch with:");
    println!("  log stream --predicate 'subsystem == \"fish.doom.apple-log\"'\n");

    let logger = Logger::new("fish.doom.apple-log", "smoke")?;
    logger.info("hello from apple-log");
    logger.debug("this is a debug line");
    logger.log(Level::Default, "default-level message");
    logger.error("oh no something went wrong");
    logger.fault("everything is on fire");

    // Free-function path (no subsystem):
    log(Level::Info, "free-function info line via OS_LOG_DEFAULT");

    println!("OK 6 messages emitted");
    Ok(())
}
