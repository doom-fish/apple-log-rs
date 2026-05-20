use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let null = OSActivity::null();
    let null_ids = null.identifiers();
    println!(
        "null activity current={} parent={:?}",
        null_ids.current, null_ids.parent
    );

    let activity = OSActivity::new(
        "example-activity",
        Some(&OSActivity::current()),
        OSActivityFlags::DEFAULT,
    )?;
    let scope = activity.enter()?;
    let ids = activity.identifiers();
    println!("activity current={} parent={:?}", ids.current, ids.parent);
    Logger::default().info("inside scoped activity");
    drop(scope);
    activity.apply(|| Logger::default().info("inside apply"));

    OSActivity::initiate("example-initiate", OSActivityFlags::DEFAULT, || {
        Logger::default().info("inside initiate");
    })?;

    let mut initiate_message = String::from("inside initiate_f");
    OSActivity::initiate_f(
        "example-initiate-f",
        OSActivityFlags::DEFAULT,
        &mut initiate_message,
        |message| Logger::default().info(message.as_str()),
    )?;

    #[cfg(feature = "async")]
    {
        let async_message = pollster::block_on(
            OSActivity::new(
                "example-async-activity",
                Some(&OSActivity::current()),
                OSActivityFlags::DEFAULT,
            )?
            .instrument_future(async {
                Logger::default().info("inside async future");
                "inside async future"
            }),
        )?;
        Logger::default().info(async_message);
    }

    OSActivity::label_user_action("example action");
    OSActivity::set_breadcrumb("example breadcrumb");
    Ok(())
}
