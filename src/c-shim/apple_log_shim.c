// apple-log C shim
//
// Apple's `os_log` / `os_signpost` / `os_activity` interfaces are exposed
// primarily as preprocessor macros that rely on this translation unit's
// `__dso_handle`. Rust can't invoke those macros directly, so we wrap the
// pieces we need in real C functions.

#include <os/activity.h>
#include <os/log.h>
#include <os/signpost.h>
#include <string.h>

os_log_t apple_log_create(const char *subsystem, const char *category) {
    return os_log_create(subsystem, category);
}

os_log_t apple_log_default(void) {
    return OS_LOG_DEFAULT;
}

os_log_t apple_log_disabled(void) {
    return OS_LOG_DISABLED;
}

void apple_log_release(os_log_t log) {
    extern void os_release(void *object);
    if (log) os_release((void *)log);
}

void apple_log_emit_privacy(os_log_t log, int level, const char *message, bool is_public) {
    if (!log || !message) return;
    os_log_type_t type = (os_log_type_t)level;
    if (is_public) {
        os_log_with_type(log, type, "%{public}s", message);
    } else {
        os_log_with_type(log, type, "%{private}s", message);
    }
}

void apple_log_emit(os_log_t log, int level, const char *message) {
    apple_log_emit_privacy(log, level, message, true);
}

void apple_log_emit_default_privacy(int level, const char *message, bool is_public) {
    if (!message) return;
    apple_log_emit_privacy(OS_LOG_DEFAULT, level, message, is_public);
}

void apple_log_emit_default(int level, const char *message) {
    apple_log_emit_default_privacy(level, message, true);
}

bool apple_log_type_enabled(os_log_t log, int level) {
    if (!log) log = OS_LOG_DEFAULT;
    return os_log_type_enabled(log, (os_log_type_t)level);
}

uint64_t apple_signpost_id_generate(os_log_t log) {
    if (!log) log = OS_LOG_DEFAULT;
    return (uint64_t)os_signpost_id_generate(log);
}

uint64_t apple_signpost_id_make_with_pointer(os_log_t log, const void *ptr) {
    if (!log) log = OS_LOG_DEFAULT;
    return (uint64_t)os_signpost_id_make_with_pointer(log, ptr);
}

bool apple_signpost_enabled(os_log_t log) {
    if (!log) log = OS_LOG_DEFAULT;
    return os_signpost_enabled(log);
}

void apple_signpost_event_emit(os_log_t log, uint64_t spid, const char *name, const char *message) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "event";
    os_signpost_event_emit(log, (os_signpost_id_t)spid, "rust", "%{public}s %{public}s",
                           name, message ? message : "");
}

void apple_signpost_interval_begin(os_log_t log, uint64_t spid, const char *name) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "interval";
    os_signpost_interval_begin(log, (os_signpost_id_t)spid, "rust", "%{public}s", name);
}

void apple_signpost_animation_interval_begin(os_log_t log, uint64_t spid, const char *name) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "animation";
    os_signpost_animation_interval_begin(log, (os_signpost_id_t)spid, "rust", "%{public}s", name);
}

void apple_signpost_interval_end(os_log_t log, uint64_t spid, const char *name) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "interval";
    os_signpost_interval_end(log, (os_signpost_id_t)spid, "rust", "%{public}s", name);
}

uint64_t apple_activity_get_identifiers(uint64_t *parent_id) {
    os_activity_id_t parent = 0;
    os_activity_id_t current = os_activity_get_identifier(
        OS_ACTIVITY_CURRENT,
        parent_id ? &parent : NULL);
    if (parent_id) {
        *parent_id = (uint64_t)parent;
    }
    return (uint64_t)current;
}

uint64_t apple_activity_get_active_id(void) {
    return apple_activity_get_identifiers(NULL);
}
