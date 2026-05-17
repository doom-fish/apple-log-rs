import re

# Based on v1 audit, these are the symbols that should be present
v1_symbols = [
    # os/log.h
    ('os_log_t', 'typedef', 'usr/include/os/log.h'),
    ('OS_LOG_DISABLED', 'macro constant', 'usr/include/os/log.h'),
    ('OS_LOG_DEFAULT', 'macro constant', 'usr/include/os/log.h'),
    ('os_log_type_t', 'enum typedef', 'usr/include/os/log.h'),
    ('OS_LOG_TYPE_DEFAULT', 'enum constant', 'usr/include/os/log.h'),
    ('OS_LOG_TYPE_INFO', 'enum constant', 'usr/include/os/log.h'),
    ('OS_LOG_TYPE_DEBUG', 'enum constant', 'usr/include/os/log.h'),
    ('OS_LOG_TYPE_ERROR', 'enum constant', 'usr/include/os/log.h'),
    ('OS_LOG_TYPE_FAULT', 'enum constant', 'usr/include/os/log.h'),
    ('os_log_create', 'function', 'usr/include/os/log.h'),
    ('os_log_info_enabled', 'macro', 'usr/include/os/log.h'),
    ('os_log_debug_enabled', 'macro', 'usr/include/os/log.h'),
    ('os_log_with_type', 'macro', 'usr/include/os/log.h'),
    ('os_log', 'macro', 'usr/include/os/log.h'),
    ('os_log_info', 'macro', 'usr/include/os/log.h'),
    ('os_log_debug', 'macro', 'usr/include/os/log.h'),
    ('os_log_error', 'macro', 'usr/include/os/log.h'),
    ('os_log_fault', 'macro', 'usr/include/os/log.h'),
    ('os_log_type_enabled', 'function', 'usr/include/os/log.h'),
    
    # os/signpost.h
    ('os_signpost_id_t', 'typedef', 'usr/include/os/signpost.h'),
    ('OS_SIGNPOST_ID_NULL', 'macro constant', 'usr/include/os/signpost.h'),
    ('OS_SIGNPOST_ID_INVALID', 'macro constant', 'usr/include/os/signpost.h'),
    ('OS_SIGNPOST_ID_EXCLUSIVE', 'macro constant', 'usr/include/os/signpost.h'),
    ('os_signpost_id_make_with_pointer', 'function', 'usr/include/os/signpost.h'),
    ('os_signpost_id_generate', 'function', 'usr/include/os/signpost.h'),
    ('os_signpost_enabled', 'function', 'usr/include/os/signpost.h'),
    ('os_signpost_interval_begin', 'macro', 'usr/include/os/signpost.h'),
    ('os_signpost_animation_interval_begin', 'macro', 'usr/include/os/signpost.h'),
    ('os_signpost_interval_end', 'macro', 'usr/include/os/signpost.h'),
    ('os_signpost_event_emit', 'macro', 'usr/include/os/signpost.h'),
    ('OS_LOG_CATEGORY_POINTS_OF_INTEREST', 'macro constant', 'usr/include/os/signpost.h'),
    ('OS_LOG_CATEGORY_DYNAMIC_TRACING', 'macro constant', 'usr/include/os/signpost.h'),
    ('OS_LOG_CATEGORY_DYNAMIC_STACK_TRACING', 'macro constant', 'usr/include/os/signpost.h'),
    
    # os/activity.h
    ('os_activity_flag_t', 'enum typedef', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_FLAG_DEFAULT', 'enum constant', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_FLAG_DETACHED', 'enum constant', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_FLAG_IF_NONE_PRESENT', 'enum constant', 'usr/include/os/activity.h'),
    ('os_activity_t', 'typedef', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_NONE', 'macro constant', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_NULL', 'macro constant', 'usr/include/os/activity.h'),
    ('OS_ACTIVITY_CURRENT', 'macro constant', 'usr/include/os/activity.h'),
    ('os_activity_id_t', 'typedef', 'usr/include/os/activity.h'),
    ('os_activity_scope_state_t', 'struct typedef', 'usr/include/os/activity.h'),
    ('os_activity_create', 'macro', 'usr/include/os/activity.h'),
    ('os_activity_initiate', 'macro', 'usr/include/os/activity.h'),
    ('os_activity_initiate_f', 'macro', 'usr/include/os/activity.h'),
    ('os_activity_apply', 'function', 'usr/include/os/activity.h'),
    ('os_activity_apply_f', 'function', 'usr/include/os/activity.h'),
    ('os_activity_scope_enter', 'function', 'usr/include/os/activity.h'),
    ('os_activity_scope_leave', 'function', 'usr/include/os/activity.h'),
    ('os_activity_scope', 'macro', 'usr/include/os/activity.h'),
    ('os_activity_get_identifier', 'function', 'usr/include/os/activity.h'),
    ('os_activity_label_useraction', 'macro', 'usr/include/os/activity.h'),
]

# EXEMPT (deprecated) from v1
v1_exempt = [
    ('os_log_is_enabled', 'function', 'usr/include/os/log.h'),
    ('os_log_is_debug_enabled', 'function', 'usr/include/os/log.h'),
    ('os_log_sensitive', 'macro', 'usr/include/os/log.h'),
    ('os_log_sensitive_debug', 'macro', 'usr/include/os/log.h'),
    ('os_activity_get_active', 'function', 'usr/include/os/activity.h'),
    ('os_activity_start', 'macro', 'usr/include/os/activity.h'),
    ('os_activity_end', 'function', 'usr/include/os/activity.h'),
    ('os_breadcrumb_t', 'typedef', 'usr/include/os/activity.h'),
    ('os_activity_set_breadcrumb', 'macro', 'usr/include/os/activity.h'),
]

print(f"V1 VERIFIED: {len(v1_symbols)}")
print(f"V1 EXEMPT: {len(v1_exempt)}")
print(f"Total SDK symbols in v1: {len(v1_symbols) + len(v1_exempt)}")
