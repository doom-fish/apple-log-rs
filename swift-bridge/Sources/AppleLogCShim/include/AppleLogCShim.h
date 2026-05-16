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

void *apple_os_atomic_queue_create(void);
void apple_os_atomic_queue_destroy(void *queue);
void apple_os_atomic_queue_enqueue_value(void *queue, uintptr_t value);
bool apple_os_atomic_queue_dequeue_value(void *queue, uintptr_t *value_out);

void *apple_os_atomic_fifo_queue_create(void);
void apple_os_atomic_fifo_queue_destroy(void *queue);
void apple_os_atomic_fifo_enqueue_value(void *queue, uintptr_t value);
bool apple_os_atomic_fifo_dequeue_value(void *queue, uintptr_t *value_out);

int32_t apple_os_atomic_add32(int32_t amount, volatile int32_t *value);
int32_t apple_os_atomic_add32_barrier(int32_t amount, volatile int32_t *value);
int32_t apple_os_atomic_increment32(volatile int32_t *value);
int32_t apple_os_atomic_increment32_barrier(volatile int32_t *value);
int32_t apple_os_atomic_decrement32(volatile int32_t *value);
int32_t apple_os_atomic_decrement32_barrier(volatile int32_t *value);
int32_t apple_os_atomic_or32(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_or32_barrier(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_or32_orig(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_or32_orig_barrier(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_and32(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_and32_barrier(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_and32_orig(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_and32_orig_barrier(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_xor32(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_xor32_barrier(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_xor32_orig(uint32_t mask, volatile uint32_t *value);
int32_t apple_os_atomic_xor32_orig_barrier(uint32_t mask, volatile uint32_t *value);
bool apple_os_atomic_compare_and_swap32(int32_t old_value, int32_t new_value, volatile int32_t *value);
bool apple_os_atomic_compare_and_swap32_barrier(int32_t old_value, int32_t new_value, volatile int32_t *value);

int64_t apple_os_atomic_add64(int64_t amount, volatile int64_t *value);
int64_t apple_os_atomic_add64_barrier(int64_t amount, volatile int64_t *value);
int64_t apple_os_atomic_increment64(volatile int64_t *value);
int64_t apple_os_atomic_increment64_barrier(volatile int64_t *value);
int64_t apple_os_atomic_decrement64(volatile int64_t *value);
int64_t apple_os_atomic_decrement64_barrier(volatile int64_t *value);
bool apple_os_atomic_compare_and_swap64(int64_t old_value, int64_t new_value, volatile int64_t *value);
bool apple_os_atomic_compare_and_swap64_barrier(int64_t old_value, int64_t new_value, volatile int64_t *value);

bool apple_os_atomic_compare_and_swap_int(int old_value, int new_value, volatile int *value);
bool apple_os_atomic_compare_and_swap_int_barrier(int old_value, int new_value, volatile int *value);
bool apple_os_atomic_compare_and_swap_long(long old_value, long new_value, volatile long *value);
bool apple_os_atomic_compare_and_swap_long_barrier(long old_value, long new_value, volatile long *value);
bool apple_os_atomic_compare_and_swap_ptr(void *old_value, void *new_value, void *volatile *value);
bool apple_os_atomic_compare_and_swap_ptr_barrier(void *old_value, void *new_value, void *volatile *value);
bool apple_os_atomic_test_and_set(uint32_t bit, volatile void *address);
bool apple_os_atomic_test_and_set_barrier(uint32_t bit, volatile void *address);
bool apple_os_atomic_test_and_clear(uint32_t bit, volatile void *address);
bool apple_os_atomic_test_and_clear_barrier(uint32_t bit, volatile void *address);

#ifdef __cplusplus
}
#endif
