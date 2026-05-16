use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = OSLog::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST)?;
    println!("subsystem={:?} category={:?}", log.subsystem(), log.category());
    println!("info enabled={}", log.is_enabled(Level::Info));
    println!("signposts enabled={}", log.signposts_enabled());
    let _ = OSLog::default();
    let _ = OSLog::disabled();
    Ok(())
}
