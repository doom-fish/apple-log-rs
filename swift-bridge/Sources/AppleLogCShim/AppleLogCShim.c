#include "AppleLogCShim.h"

#include <os/activity.h>
#include <os/log.h>
#include <os/signpost.h>
#include <stdlib.h>
#include <string.h>

extern void os_release(void *object);

#define APPLE_ACTIVITY_NULL_SENTINEL ((apple_activity_handle_t)(uintptr_t)1)

static bool apple_is_null_activity(apple_activity_handle_t activity) {
    return activity == APPLE_ACTIVITY_NULL_SENTINEL;
}

static os_log_t apple_resolve_log(apple_log_handle_t log) {
    return log ? (os_log_t)log : OS_LOG_DEFAULT;
}

static os_activity_t apple_resolve_activity(apple_activity_handle_t activity) {
    if (!activity) {
        return OS_ACTIVITY_CURRENT;
    }
    if (apple_is_null_activity(activity)) {
        return OS_ACTIVITY_NULL;
    }
    return (os_activity_t)activity;
}

apple_log_handle_t apple_log_create(const char *subsystem, const char *category) {
    return (apple_log_handle_t)os_log_create(subsystem, category);
}

apple_log_handle_t apple_log_default(void) {
    return (apple_log_handle_t)OS_LOG_DEFAULT;
}

apple_log_handle_t apple_log_disabled(void) {
    return (apple_log_handle_t)OS_LOG_DISABLED;
}

void apple_log_release(apple_log_handle_t log) {
    if (log) {
        os_release(log);
    }
}

void apple_log_emit_privacy(apple_log_handle_t log, int32_t level, const char *message, bool is_public) {
    if (!message) {
        return;
    }
    os_log_type_t type = (os_log_type_t)level;
    if (is_public) {
        os_log_with_type(apple_resolve_log(log), type, "%{public}s", message);
    } else {
        os_log_with_type(apple_resolve_log(log), type, "%{private}s", message);
    }
}

void apple_log_emit(apple_log_handle_t log, int32_t level, const char *message) {
    apple_log_emit_privacy(log, level, message, false);
}

void apple_log_emit_default_privacy(int32_t level, const char *message, bool is_public) {
    apple_log_emit_privacy(OS_LOG_DEFAULT, level, message, is_public);
}

void apple_log_emit_default(int32_t level, const char *message) {
    apple_log_emit_privacy(OS_LOG_DEFAULT, level, message, false);
}

bool apple_log_type_enabled(apple_log_handle_t log, int32_t level) {
    return os_log_type_enabled(apple_resolve_log(log), (os_log_type_t)level);
}

uint64_t apple_signpost_id_generate(apple_log_handle_t log) {
    return (uint64_t)os_signpost_id_generate(apple_resolve_log(log));
}

uint64_t apple_signpost_id_make_with_pointer(apple_log_handle_t log, const void *ptr) {
    return (uint64_t)os_signpost_id_make_with_pointer(apple_resolve_log(log), ptr);
}

bool apple_signpost_enabled(apple_log_handle_t log) {
    return os_signpost_enabled(apple_resolve_log(log));
}

#define APPLE_SIGNPOST_EMIT(log, type, spid, name, fmt, ...) __extension__({ \
        OS_LOG_PRAGMA_PUSH OS_LOG_STRING(LOG, _apple_fmt_str, fmt); \
        uint8_t _Alignas(16) OS_LOG_UNINITIALIZED _apple_fmt_buf[__builtin_os_log_format_buffer_size(fmt, ##__VA_ARGS__)]; \
        _os_signpost_emit_with_name_impl(&__dso_handle, (log), (type), (spid), (name), _apple_fmt_str, \
                (uint8_t *)__builtin_os_log_format(_apple_fmt_buf, fmt, ##__VA_ARGS__), \
                (uint32_t)sizeof(_apple_fmt_buf)) OS_LOG_PRAGMA_POP; \
})

void apple_signpost_emit(apple_log_handle_t log, uint64_t spid, int32_t kind, const char *name, const char *message, bool is_public) {
    os_log_t resolved = apple_resolve_log(log);
    os_signpost_id_t signpost_id = (os_signpost_id_t)spid;
    if (signpost_id == OS_SIGNPOST_ID_NULL || signpost_id == OS_SIGNPOST_ID_INVALID || !os_signpost_enabled(resolved)) {
        return;
    }
    os_signpost_type_t type;
    switch (kind) {
    case APPLE_SIGNPOST_KIND_EVENT:
        type = OS_SIGNPOST_EVENT;
        name = name ? name : "event";
        break;
    case APPLE_SIGNPOST_KIND_INTERVAL_BEGIN:
        type = OS_SIGNPOST_INTERVAL_BEGIN;
        name = name ? name : "interval";
        break;
    case APPLE_SIGNPOST_KIND_ANIMATION_INTERVAL_BEGIN:
        type = OS_SIGNPOST_INTERVAL_BEGIN;
        name = name ? name : "animation";
        break;
    case APPLE_SIGNPOST_KIND_INTERVAL_END:
        type = OS_SIGNPOST_INTERVAL_END;
        name = name ? name : "interval";
        break;
    default:
        return;
    }
    bool animation = kind == APPLE_SIGNPOST_KIND_ANIMATION_INTERVAL_BEGIN;
    if (!message) {
        if (animation) {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, " " _OS_SIGNPOST_ANIMATION_INTERVAL_TAG);
        } else {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, "");
        }
    } else if (is_public) {
        if (animation) {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, "%{public}s " _OS_SIGNPOST_ANIMATION_INTERVAL_TAG, message);
        } else {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, "%{public}s", message);
        }
    } else {
        if (animation) {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, "%{private}s " _OS_SIGNPOST_ANIMATION_INTERVAL_TAG, message);
        } else {
            APPLE_SIGNPOST_EMIT(resolved, type, signpost_id, name, "%{private}s", message);
        }
    }
}

void apple_signpost_event_emit(apple_log_handle_t log, uint64_t spid, const char *name, const char *message) {
    apple_signpost_emit(log, spid, APPLE_SIGNPOST_KIND_EVENT, name, message, false);
}

void apple_signpost_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name) {
    apple_signpost_emit(log, spid, APPLE_SIGNPOST_KIND_INTERVAL_BEGIN, name, NULL, false);
}

void apple_signpost_animation_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name) {
    apple_signpost_emit(log, spid, APPLE_SIGNPOST_KIND_ANIMATION_INTERVAL_BEGIN, name, NULL, false);
}

void apple_signpost_interval_end(apple_log_handle_t log, uint64_t spid, const char *name) {
    apple_signpost_emit(log, spid, APPLE_SIGNPOST_KIND_INTERVAL_END, name, NULL, false);
}

uint64_t apple_activity_get_identifiers(uint64_t *parent_id) {
    os_activity_id_t parent = 0;
    os_activity_id_t current = os_activity_get_identifier(OS_ACTIVITY_CURRENT, parent_id ? &parent : NULL);
    if (parent_id) {
        *parent_id = (uint64_t)parent;
    }
    return (uint64_t)current;
}

uint64_t apple_activity_get_active_id(void) {
    return apple_activity_get_identifiers(NULL);
}

apple_activity_handle_t apple_activity_create(const char *description, apple_activity_handle_t parent_activity, uint32_t flags) {
    return (apple_activity_handle_t)_os_activity_create(&__dso_handle, description ? description : "activity", apple_resolve_activity(parent_activity), (os_activity_flag_t)flags);
}

apple_activity_handle_t apple_activity_start(const char *description, uint32_t flags) {
    return (apple_activity_handle_t)_os_activity_start(&__dso_handle, description ? description : "activity", (os_activity_flag_t)flags);
}

apple_activity_handle_t apple_activity_current(void) {
    return (apple_activity_handle_t)OS_ACTIVITY_CURRENT;
}

apple_activity_handle_t apple_activity_none(void) {
    return (apple_activity_handle_t)OS_ACTIVITY_NONE;
}

apple_activity_handle_t apple_activity_null(void) {
    return APPLE_ACTIVITY_NULL_SENTINEL;
}

void apple_activity_release(apple_activity_handle_t activity) {
    if (!activity || apple_is_null_activity(activity) || activity == (apple_activity_handle_t)OS_ACTIVITY_CURRENT || activity == (apple_activity_handle_t)OS_ACTIVITY_NONE) {
        return;
    }
    os_release(activity);
}

void apple_activity_initiate_f(const char *description, uint32_t flags, void *context, apple_log_function_t function) {
    if (!function) {
        return;
    }
    _os_activity_initiate_f(&__dso_handle, description ? description : "activity", (os_activity_flag_t)flags, context, (os_function_t)function);
}

void apple_activity_apply_f(apple_activity_handle_t activity, void *context, apple_log_function_t function) {
    if (!function) {
        return;
    }
    if (apple_is_null_activity(activity)) {
        function(context);
        return;
    }
    os_activity_apply_f(apple_resolve_activity(activity), context, (os_function_t)function);
}

typedef struct apple_activity_scope_box_s {
    bool uses_scope;
    struct os_activity_scope_state_s state;
} apple_activity_scope_box_t;

apple_activity_scope_handle_t apple_activity_scope_enter_alloc(apple_activity_handle_t activity) {
    apple_activity_scope_box_t *box = malloc(sizeof(*box));
    if (!box) {
        return NULL;
    }
    if (apple_is_null_activity(activity)) {
        box->uses_scope = false;
        memset(&box->state, 0, sizeof(box->state));
        return (apple_activity_scope_handle_t)box;
    }
    box->uses_scope = true;
    os_activity_scope_enter(apple_resolve_activity(activity), &box->state);
    return (apple_activity_scope_handle_t)box;
}

void apple_activity_scope_leave_free(apple_activity_scope_handle_t state) {
    apple_activity_scope_box_t *box = (apple_activity_scope_box_t *)state;
    if (!box) {
        return;
    }
    if (box->uses_scope) {
        os_activity_scope_leave(&box->state);
    }
    free(box);
}

uint64_t apple_activity_get_identifier(apple_activity_handle_t activity, uint64_t *parent_id) {
    if (apple_is_null_activity(activity)) {
        if (parent_id) {
            *parent_id = 0;
        }
        return 0;
    }

    os_activity_id_t parent = 0;
    os_activity_id_t current = os_activity_get_identifier(apple_resolve_activity(activity), parent_id ? &parent : NULL);
    if (parent_id) {
        *parent_id = (uint64_t)parent;
    }
    return (uint64_t)current;
}

void apple_activity_end(apple_activity_handle_t activity) {
    if (activity && !apple_is_null_activity(activity)) {
        os_activity_end((os_activity_t)activity);
    }
}

void apple_activity_label_useraction(const char *label) {
    _os_activity_label_useraction(&__dso_handle, label ? label : "user-action");
}

void apple_activity_set_breadcrumb(const char *name) {
    _os_activity_set_breadcrumb(&__dso_handle, name ? name : "breadcrumb");
}
