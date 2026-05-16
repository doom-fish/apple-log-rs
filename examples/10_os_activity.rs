use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let activity = OSActivity::new("example-activity", Some(&OSActivity::current()), OSActivityFlags::DEFAULT)?;
    let scope = activity.enter()?;
    let ids = activity.identifiers();
    println!("activity current={} parent={:?}", ids.current, ids.parent);
    Logger::default().info("inside scoped activity");
    drop(scope);
    activity.apply(|| Logger::default().info("inside apply"));
    OSActivity::label_user_action("example action");
    OSActivity::set_breadcrumb("example breadcrumb");
    Ok(())
}
