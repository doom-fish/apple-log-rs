// apple-log C shim
//
// Apple's `os_log` interface is exposed primarily as preprocessor macros
// (`os_log`, `os_log_with_type`, …) that expand to `_os_log_internal`
// with a per-binary `__dso_handle` pointer. Rust can't invoke macros and
// doesn't have its own `__dso_handle`, so we wrap the macros in real
// C functions that *this* C translation unit can call.

#include <os/log.h>
#include <string.h>

// Create a new os_log_t handle for the given subsystem + category.
os_log_t apple_log_create(const char *subsystem, const char *category) {
    return os_log_create(subsystem, category);
}

// Drop (release) an os_log_t handle.
void apple_log_release(os_log_t log) {
    // os_log_t is a heap-allocated CFType-like; the only documented way
    // to release it is os_release(), which is itself a macro that wraps
    // os_retain_release() on macOS.
    extern void os_release(void *object);
    if (log) os_release((void *)log);
}

// Log `message` (a NUL-terminated C string) at the requested level.
// Levels mirror `os_log_type_t`:
//   0 = DEFAULT, 1 = INFO, 2 = DEBUG, 16 = ERROR, 17 = FAULT.
void apple_log_emit(os_log_t log, int level, const char *message) {
    if (!log || !message) return;
    os_log_type_t type = (os_log_type_t)level;
    // %{public}s prevents the system log from redacting our text.
    os_log_with_type(log, type, "%{public}s", message);
}

// Use OS_LOG_DEFAULT (the catch-all process logger) — handy when the
// caller doesn't care about subsystem/category.
void apple_log_emit_default(int level, const char *message) {
    if (!message) return;
    os_log_type_t type = (os_log_type_t)level;
    os_log_with_type(OS_LOG_DEFAULT, type, "%{public}s", message);
}

// --- Type-enabled check (v0.2) ---

bool apple_log_type_enabled(os_log_t log, int level) {
    if (!log) log = OS_LOG_DEFAULT;
    return os_log_type_enabled(log, (os_log_type_t)level);
}

// --- Signposts (v0.2) ---
// os_signpost_id_t is uint64_t; OS_SIGNPOST_ID_INVALID = ~0ULL.

#include <os/signpost.h>

uint64_t apple_signpost_id_generate(os_log_t log) {
    if (!log) log = OS_LOG_DEFAULT;
    return (uint64_t)os_signpost_id_generate(log);
}

bool apple_signpost_enabled(os_log_t log) {
    if (!log) log = OS_LOG_DEFAULT;
    return os_signpost_enabled(log);
}

void apple_signpost_event_emit(os_log_t log, uint64_t spid, const char *name, const char *message) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "event";
    // Constant strings only — Apple's macros bake the name in at compile
    // time. We accept dynamic names by using %{public}s formatting.
    os_signpost_event_emit(log, (os_signpost_id_t)spid, "rust", "%{public}s %{public}s",
                            name, message ? message : "");
}

void apple_signpost_interval_begin(os_log_t log, uint64_t spid, const char *name) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "interval";
    os_signpost_interval_begin(log, (os_signpost_id_t)spid, "rust", "%{public}s", name);
}

void apple_signpost_interval_end(os_log_t log, uint64_t spid, const char *name) {
    if (!log) log = OS_LOG_DEFAULT;
    if (!name) name = "interval";
    os_signpost_interval_end(log, (os_signpost_id_t)spid, "rust", "%{public}s", name);
}

// --- Activity (v0.3) ---
// Apple's full os_activity_scope_state_s requires per-scope state on
// the caller's stack which doesn't bridge cleanly to Rust drop order.
// We expose only the readable id of the currently-active activity —
// that's enough to thread an activity id through logs without
// risking incorrect scope nesting.

#include <os/activity.h>

/// Return the integer id of the currently-active activity (0 if none).
/// Useful for stamping log messages with an activity correlation id.
uint64_t apple_activity_get_active_id(void) {
    return (uint64_t)os_activity_get_identifier(
        OS_ACTIVITY_CURRENT, NULL);
}
