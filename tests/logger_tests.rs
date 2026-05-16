use apple_log::prelude::*;

#[test]
fn logger_smoke() {
    let logger = Logger::new("fish.doom.apple-log", "logger-tests").expect("logger");
    logger.trace("trace");
    logger.debug("debug");
    logger.info("info");
    logger.notice("notice");
    logger.warning("warning");
    logger.error("error");
    logger.critical("critical");
    logger.fault("fault");
    logger.log_with_privacy(Level::Info, "private", Privacy::Private);
    let _ = logger.is_enabled(Level::Info);
    let _ = Logger::default();
    let _ = Logger::disabled();
}
