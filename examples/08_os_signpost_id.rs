use apple_log::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = OSLog::new("fish.doom.apple-log", CATEGORY_POINTS_OF_INTEREST)?;
    let generated = OSSignpostId::generate(&log);
    let from_pointer = OSSignpostId::from_pointer(&log, std::ptr::addr_of!(log));
    println!("null={} invalid={} exclusive={}", OSSignpostId::NULL.as_u64(), OSSignpostId::INVALID.as_u64(), OSSignpostId::EXCLUSIVE.as_u64());
    println!("generated={} from_pointer={}", generated.as_u64(), from_pointer.as_u64());
    Ok(())
}
