import re
import os

# Map of SDK symbols to what they should be wrapped by in the crate
coverage_map = {
    'os_log_t': ['OSLog', 'Logger'],
    'OS_LOG_DEFAULT': ['OSLog::default', 'log', 'Logger::default'],
    'OS_LOG_DISABLED': ['OSLog::disabled', 'Logger::disabled'],
    'os_log_type_t': ['Level'],
    'OS_LOG_TYPE_DEFAULT': ['Level::Default'],
    'OS_LOG_TYPE_INFO': ['Level::Info'],
    'OS_LOG_TYPE_DEBUG': ['Level::Debug'],
    'OS_LOG_TYPE_ERROR': ['Level::Error'],
    'OS_LOG_TYPE_FAULT': ['Level::Fault'],
    'os_log_create': ['OSLog::new', 'Logger::new'],
    'os_log_type_enabled': ['OSLog::is_enabled', 'Logger::is_enabled'],
    'os_signpost_id_t': ['OSSignpostId', 'SignpostId'],
    'os_signpost_id_generate': ['OSSignpostId::generate', 'OSSignposter::make_signpost_id'],
    'os_signpost_id_make_with_pointer': ['OSSignpostId::from_pointer'],
    'os_signpost_enabled': ['OSSignposter::is_enabled'],
    'os_activity_flag_t': ['OSActivityFlags'],
    'OS_ACTIVITY_FLAG_DEFAULT': ['OSActivityFlags::DEFAULT'],
    'OS_ACTIVITY_FLAG_DETACHED': ['OSActivityFlags::DETACHED'],
    'OS_ACTIVITY_FLAG_IF_NONE_PRESENT': ['OSActivityFlags::IF_NONE_PRESENT'],
    'os_activity_t': ['OSActivity'],
    'OS_ACTIVITY_NONE': ['OSActivity::none'],
    'OS_ACTIVITY_NULL': ['OSActivity::null'],
    'OS_ACTIVITY_CURRENT': ['OSActivity::current'],
    'os_activity_create': ['OSActivity::new'],
    'os_activity_apply': ['OSActivity::apply'],
    'os_activity_scope_enter': ['OSActivity::enter'],
    'os_activity_scope_leave': ['OSActivityScope'],
    'os_activity_get_identifier': ['OSActivity::identifier'],
}

# Read all Rust source files
rust_content = ""
for root, dirs, files in os.walk('src'):
    for f in files:
        if f.endswith('.rs'):
            with open(os.path.join(root, f), 'r') as file:
                rust_content += file.read() + "\n"

# Read all Swift files
swift_content = ""
for root, dirs, files in os.walk('swift-bridge'):
    for f in files:
        if f.endswith('.swift'):
            with open(os.path.join(root, f), 'r') as file:
                swift_content += file.read() + "\n"

print("Coverage verification:")
verified_count = 0
for sdk_sym, crate_items in sorted(coverage_map.items()):
    found = False
    for item in crate_items:
        if item in rust_content or item in swift_content:
            found = True
            break
    
    status = "✓" if found else "✗"
    print(f"{status} {sdk_sym:40} -> {', '.join(crate_items)}")
    if found:
        verified_count += 1

print(f"\nVerified coverage: {verified_count}/{len(coverage_map)}")
