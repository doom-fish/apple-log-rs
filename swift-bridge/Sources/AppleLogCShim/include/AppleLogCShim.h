#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void *apple_log_handle_t;
typedef void *apple_activity_handle_t;
typedef void *apple_activity_scope_handle_t;
typedef void (*apple_log_function_t)(void *context);

enum {
    APPLE_SIGNPOST_KIND_EVENT = 0,
    APPLE_SIGNPOST_KIND_INTERVAL_BEGIN = 1,
    APPLE_SIGNPOST_KIND_ANIMATION_INTERVAL_BEGIN = 2,
    APPLE_SIGNPOST_KIND_INTERVAL_END = 3,
};

apple_log_handle_t apple_log_create(const char *subsystem, const char *category);
apple_log_handle_t apple_log_default(void);
apple_log_handle_t apple_log_disabled(void);
void apple_log_release(apple_log_handle_t log);
void apple_log_emit(apple_log_handle_t log, int32_t level, const char *message);
void apple_log_emit_privacy(apple_log_handle_t log, int32_t level, const char *message, bool is_public);
void apple_log_emit_default(int32_t level, const char *message);
void apple_log_emit_default_privacy(int32_t level, const char *message, bool is_public);
bool apple_log_type_enabled(apple_log_handle_t log, int32_t level);

uint64_t apple_signpost_id_generate(apple_log_handle_t log);
uint64_t apple_signpost_id_make_with_pointer(apple_log_handle_t log, const void *ptr);
bool apple_signpost_enabled(apple_log_handle_t log);
void apple_signpost_emit(apple_log_handle_t log, uint64_t spid, int32_t kind, const char *name, const char *message, bool is_public);
void apple_signpost_event_emit(apple_log_handle_t log, uint64_t spid, const char *name, const char *message);
void apple_signpost_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name);
void apple_signpost_animation_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name);
void apple_signpost_interval_end(apple_log_handle_t log, uint64_t spid, const char *name);

uint64_t apple_activity_get_identifiers(uint64_t *parent_id);
uint64_t apple_activity_get_active_id(void);
apple_activity_handle_t apple_activity_create(const char *description, apple_activity_handle_t parent_activity, uint32_t flags);
apple_activity_handle_t apple_activity_start(const char *description, uint32_t flags);
apple_activity_handle_t apple_activity_current(void);
apple_activity_handle_t apple_activity_none(void);
apple_activity_handle_t apple_activity_null(void);
void apple_activity_release(apple_activity_handle_t activity);
void apple_activity_initiate_f(const char *description, uint32_t flags, void *context, apple_log_function_t function);
void apple_activity_apply_f(apple_activity_handle_t activity, void *context, apple_log_function_t function);
apple_activity_scope_handle_t apple_activity_scope_enter_alloc(apple_activity_handle_t activity);
void apple_activity_scope_leave_free(apple_activity_scope_handle_t state);
uint64_t apple_activity_get_identifier(apple_activity_handle_t activity, uint64_t *parent_id);
void apple_activity_end(apple_activity_handle_t activity);
void apple_activity_label_useraction(const char *label);
void apple_activity_set_breadcrumb(const char *name);

#ifdef __cplusplus
}
#endif
