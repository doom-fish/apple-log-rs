use std::cell::Cell;

use apple_log::prelude::*;

#[test]
fn os_activity_smoke() {
    let current = OSActivity::current();
    let _ = current.identifier();
    let _ = OSActivity::none();

    let null = OSActivity::null();
    let null_ids = null.identifiers();
    assert_eq!(null_ids.current, 0);
    assert_eq!(null_ids.parent, None);

    let null_applied = Cell::new(false);
    null.apply(|| null_applied.set(true));
    assert!(null_applied.get());
    let null_scope = null.enter().expect("null scope");
    drop(null_scope);

    let activity = OSActivity::new(c"activity-test", Some(&current), OSActivityFlags::DEFAULT)
        .expect("activity");
    let ids = activity.identifiers();
    assert!(ids.current > 0 || ids.parent.is_none());
    activity.apply(|| Logger::default().info("inside apply"));
    let scope = activity.enter().expect("scope");
    Logger::default().info("inside scope");
    drop(scope);

    let initiated = Cell::new(false);
    OSActivity::initiate(c"activity-initiate", OSActivityFlags::DEFAULT, || {
        initiated.set(true);
        Logger::default().info("inside initiate");
    });
    assert!(initiated.get());

    let mut initiate_count = 0_u8;
    OSActivity::initiate_f(
        c"activity-initiate-f",
        OSActivityFlags::DEFAULT,
        &mut initiate_count,
        |count| {
            *count += 1;
            Logger::default().info("inside initiate_f");
        },
    );
    assert_eq!(initiate_count, 1);

    OSActivity::label_user_action(c"activity smoke");
    OSActivity::set_breadcrumb(c"activity breadcrumb");
}

const fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn activities_can_be_applied_on_other_threads() {
    assert_send_sync::<OSActivity>();
    let activity =
        OSActivity::new(c"cross-thread", None, OSActivityFlags::DEFAULT).expect("activity");
    let expected = activity.identifier();
    assert_ne!(expected, 0);
    let observed = std::thread::spawn(move || {
        let mut inside = 0;
        activity.apply(|| inside = active_activity_id());
        inside
    })
    .join()
    .expect("thread");
    assert_eq!(observed, expected);
}

#[cfg(feature = "async")]
#[test]
fn instrumented_futures_are_send() {
    const fn assert_send<T: Send>(_: &T) {}
    let future = OSActivity::new(c"async-send", None, OSActivityFlags::DEFAULT)
        .expect("activity")
        .instrument_future(async { 1 });
    assert_send(&future);
    assert_eq!(pollster::block_on(future).expect("poll"), 1);
}
