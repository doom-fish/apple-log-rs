use apple_log::prelude::*;

#[test]
fn os_signposter_smoke() {
    let signposter =
        OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("signposter");
    let id = signposter.make_signpost_id();
    assert!(!id.is_null() && !id.is_invalid());
    signposter.emit_event(c"event", id, "payload");
    signposter.emit_event_with_privacy(c"event", id, "public payload", Privacy::Public);
    let interval = signposter.begin_animation_interval(c"anim", id, "begin");
    signposter.end_interval(interval, "end");
    let _ = OSSignposter::default();
    assert!(!OSSignposter::disabled().is_enabled());
}

#[test]
fn intervals_remember_their_static_name() {
    let signposter =
        OSSignposter::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("signposter");
    let id = signposter.make_signpost_id();

    let interval = signposter.begin_interval(c"remembered", id, "");
    assert_eq!(interval.name(), c"remembered");
    assert_eq!(interval.id(), id);
    assert!(!interval.is_animation());
    signposter.end_interval(interval, "");

    let animation = signposter.begin_animation_interval_with_privacy(
        c"animated",
        id,
        "public",
        Privacy::Public,
    );
    assert_eq!(animation.name(), c"animated");
    assert!(animation.is_animation());
    signposter.end_interval_with_privacy(animation, "done", Privacy::Public);

    let private =
        signposter.begin_interval_with_privacy(c"private", id, "secret", Privacy::Private);
    assert_eq!(private.name(), c"private");
    signposter.end_interval(private, "");

    assert_eq!(
        signposter.with_interval_signpost(c"wrapped", id, "around", || 7),
        7
    );
}

#[test]
fn logger_signposts_use_static_names() {
    let logger = Logger::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST).expect("logger");
    let id = logger.signpost_id();
    assert!(!id.is_null() && !id.is_invalid());
    logger.signpost_interval_begin(id, c"logger-interval");
    logger.signpost_event(id, c"logger-event", "private detail");
    logger.signpost_event_with_privacy(id, c"logger-event", "public detail", Privacy::Public);
    logger.signpost_interval_end(id, c"logger-interval");
    logger.signpost_animation_interval_begin(id, c"logger-animation");
    logger.signpost_interval_end(id, c"logger-animation");
    assert!(!Logger::disabled().signposts_enabled());
}
