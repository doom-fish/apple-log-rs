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
