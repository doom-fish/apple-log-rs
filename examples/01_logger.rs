use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::new("fish.doom.apple-log", "logger")?;
    logger.trace("trace");
    logger.debug("debug");
    logger.info("info");
    logger.notice("notice");
    logger.warning("warning");
    logger.error("error");
    logger.critical("critical");
    logger.fault("fault");
    logger.log_with_privacy(Level::Info, "private payload", Privacy::Private);
    println!("logger info enabled: {}", logger.is_enabled(Level::Info));
    Ok(())
}
