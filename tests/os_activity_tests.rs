use apple_log::prelude::*;

#[test]
fn os_activity_smoke() {
    let current = OSActivity::current();
    let _ = current.identifier();
    let _ = OSActivity::none();
    let activity = OSActivity::new("activity-test", Some(&current), OSActivityFlags::DEFAULT).expect("activity");
    let ids = activity.identifiers();
    assert!(ids.current > 0 || ids.parent.is_none());
    activity.apply(|| Logger::default().info("inside apply"));
    let scope = activity.enter().expect("scope");
    Logger::default().info("inside scope");
    drop(scope);
    OSActivity::label_user_action("activity smoke");
    OSActivity::set_breadcrumb("activity breadcrumb");
}
