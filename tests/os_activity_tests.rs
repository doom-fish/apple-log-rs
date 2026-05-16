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

    let activity = OSActivity::new("activity-test", Some(&current), OSActivityFlags::DEFAULT)
        .expect("activity");
    let ids = activity.identifiers();
    assert!(ids.current > 0 || ids.parent.is_none());
    activity.apply(|| Logger::default().info("inside apply"));
    let scope = activity.enter().expect("scope");
    Logger::default().info("inside scope");
    drop(scope);

    let initiated = Cell::new(false);
    OSActivity::initiate("activity-initiate", OSActivityFlags::DEFAULT, || {
        initiated.set(true);
        Logger::default().info("inside initiate");
    })
    .expect("initiate");
    assert!(initiated.get());

    let mut initiate_count = 0_u8;
    OSActivity::initiate_f(
        "activity-initiate-f",
        OSActivityFlags::DEFAULT,
        &mut initiate_count,
        |count| {
            *count += 1;
            Logger::default().info("inside initiate_f");
        },
    )
    .expect("initiate_f");
    assert_eq!(initiate_count, 1);

    OSActivity::label_user_action("activity smoke");
    OSActivity::set_breadcrumb("activity breadcrumb");
}
