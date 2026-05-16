#include "AppleLogCShim.h"

#include <libkern/OSAtomic.h>
#include <os/activity.h>
#include <os/log.h>
#include <os/signpost.h>
#include <stdlib.h>
#include <string.h>

typedef struct apple_atomic_queue_box_s {
    OSQueueHead head;
} apple_atomic_queue_box_t;

typedef struct apple_atomic_fifo_queue_box_s {
    OSFifoQueueHead head;
} apple_atomic_fifo_queue_box_t;

typedef struct apple_atomic_queue_node_s {
    struct apple_atomic_queue_node_s *next;
    uintptr_t value;
} apple_atomic_queue_node_t;

typedef struct apple_atomic_fifo_node_s {
    struct apple_atomic_fifo_node_s *next;
    uintptr_t value;
} apple_atomic_fifo_node_t;

extern void os_release(void *object);

static os_log_t apple_resolve_log(apple_log_handle_t log) {
    return log ? (os_log_t)log : OS_LOG_DEFAULT;
}

static os_activity_t apple_resolve_activity(apple_activity_handle_t activity) {
    return activity ? (os_activity_t)activity : OS_ACTIVITY_CURRENT;
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
    apple_log_emit_privacy(log, level, message, true);
}

void apple_log_emit_default_privacy(int32_t level, const char *message, bool is_public) {
    apple_log_emit_privacy(OS_LOG_DEFAULT, level, message, is_public);
}

void apple_log_emit_default(int32_t level, const char *message) {
    apple_log_emit_privacy(OS_LOG_DEFAULT, level, message, true);
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

void apple_signpost_event_emit(apple_log_handle_t log, uint64_t spid, const char *name, const char *message) {
    const char *event_name = name ? name : "event";
    const char *event_message = message ? message : "";
    os_signpost_event_emit(apple_resolve_log(log), (os_signpost_id_t)spid, "rust", "%{public}s %{public}s", event_name, event_message);
}

void apple_signpost_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name) {
    const char *interval_name = name ? name : "interval";
    os_signpost_interval_begin(apple_resolve_log(log), (os_signpost_id_t)spid, "rust", "%{public}s", interval_name);
}

void apple_signpost_animation_interval_begin(apple_log_handle_t log, uint64_t spid, const char *name) {
    const char *interval_name = name ? name : "animation";
    os_signpost_animation_interval_begin(apple_resolve_log(log), (os_signpost_id_t)spid, "rust", "%{public}s", interval_name);
}

void apple_signpost_interval_end(apple_log_handle_t log, uint64_t spid, const char *name) {
    const char *interval_name = name ? name : "interval";
    os_signpost_interval_end(apple_resolve_log(log), (os_signpost_id_t)spid, "rust", "%{public}s", interval_name);
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

void apple_activity_release(apple_activity_handle_t activity) {
    if (!activity || activity == (apple_activity_handle_t)OS_ACTIVITY_CURRENT || activity == (apple_activity_handle_t)OS_ACTIVITY_NONE) {
        return;
    }
    os_release(activity);
}

void apple_activity_apply_f(apple_activity_handle_t activity, void *context, apple_log_function_t function) {
    if (!function) {
        return;
    }
    os_activity_apply_f(apple_resolve_activity(activity), context, (os_function_t)function);
}

apple_activity_scope_handle_t apple_activity_scope_enter_alloc(apple_activity_handle_t activity) {
    struct os_activity_scope_state_s *state = malloc(sizeof(struct os_activity_scope_state_s));
    if (!state) {
        return NULL;
    }
    os_activity_scope_enter(apple_resolve_activity(activity), state);
    return (apple_activity_scope_handle_t)state;
}

void apple_activity_scope_leave_free(apple_activity_scope_handle_t state) {
    if (!state) {
        return;
    }
    os_activity_scope_leave((os_activity_scope_state_t)state);
    free(state);
}

uint64_t apple_activity_get_identifier(apple_activity_handle_t activity, uint64_t *parent_id) {
    os_activity_id_t parent = 0;
    os_activity_id_t current = os_activity_get_identifier(apple_resolve_activity(activity), parent_id ? &parent : NULL);
    if (parent_id) {
        *parent_id = (uint64_t)parent;
    }
    return (uint64_t)current;
}

void apple_activity_end(apple_activity_handle_t activity) {
    if (activity) {
        os_activity_end((os_activity_t)activity);
    }
}

void apple_activity_label_useraction(const char *label) {
    _os_activity_label_useraction(&__dso_handle, label ? label : "user-action");
}

void apple_activity_set_breadcrumb(const char *name) {
    _os_activity_set_breadcrumb(&__dso_handle, name ? name : "breadcrumb");
}

void *apple_os_atomic_queue_create(void) {
    apple_atomic_queue_box_t *box = calloc(1, sizeof(*box));
    return box;
}

void apple_os_atomic_queue_destroy(void *queue) {
    apple_atomic_queue_box_t *box = (apple_atomic_queue_box_t *)queue;
    if (!box) {
        return;
    }
    for (;;) {
        apple_atomic_queue_node_t *node = (apple_atomic_queue_node_t *)OSAtomicDequeue(&box->head, offsetof(apple_atomic_queue_node_t, next));
        if (!node) {
            break;
        }
        free(node);
    }
    free(box);
}

void apple_os_atomic_queue_enqueue_value(void *queue, uintptr_t value) {
    apple_atomic_queue_box_t *box = (apple_atomic_queue_box_t *)queue;
    if (!box) {
        return;
    }
    apple_atomic_queue_node_t *node = calloc(1, sizeof(*node));
    if (!node) {
        return;
    }
    node->value = value;
    OSAtomicEnqueue(&box->head, node, offsetof(apple_atomic_queue_node_t, next));
}

bool apple_os_atomic_queue_dequeue_value(void *queue, uintptr_t *value_out) {
    apple_atomic_queue_box_t *box = (apple_atomic_queue_box_t *)queue;
    if (!box) {
        return false;
    }
    apple_atomic_queue_node_t *node = (apple_atomic_queue_node_t *)OSAtomicDequeue(&box->head, offsetof(apple_atomic_queue_node_t, next));
    if (!node) {
        return false;
    }
    if (value_out) {
        *value_out = node->value;
    }
    free(node);
    return true;
}

void *apple_os_atomic_fifo_queue_create(void) {
    apple_atomic_fifo_queue_box_t *box = calloc(1, sizeof(*box));
    return box;
}

void apple_os_atomic_fifo_queue_destroy(void *queue) {
    apple_atomic_fifo_queue_box_t *box = (apple_atomic_fifo_queue_box_t *)queue;
    if (!box) {
        return;
    }
    for (;;) {
        apple_atomic_fifo_node_t *node = (apple_atomic_fifo_node_t *)OSAtomicFifoDequeue(&box->head, offsetof(apple_atomic_fifo_node_t, next));
        if (!node) {
            break;
        }
        free(node);
    }
    free(box);
}

void apple_os_atomic_fifo_enqueue_value(void *queue, uintptr_t value) {
    apple_atomic_fifo_queue_box_t *box = (apple_atomic_fifo_queue_box_t *)queue;
    if (!box) {
        return;
    }
    apple_atomic_fifo_node_t *node = calloc(1, sizeof(*node));
    if (!node) {
        return;
    }
    node->value = value;
    OSAtomicFifoEnqueue(&box->head, node, offsetof(apple_atomic_fifo_node_t, next));
}

bool apple_os_atomic_fifo_dequeue_value(void *queue, uintptr_t *value_out) {
    apple_atomic_fifo_queue_box_t *box = (apple_atomic_fifo_queue_box_t *)queue;
    if (!box) {
        return false;
    }
    apple_atomic_fifo_node_t *node = (apple_atomic_fifo_node_t *)OSAtomicFifoDequeue(&box->head, offsetof(apple_atomic_fifo_node_t, next));
    if (!node) {
        return false;
    }
    if (value_out) {
        *value_out = node->value;
    }
    free(node);
    return true;
}

int32_t apple_os_atomic_add32(int32_t amount, volatile int32_t *value) { return OSAtomicAdd32(amount, value); }
int32_t apple_os_atomic_add32_barrier(int32_t amount, volatile int32_t *value) { return OSAtomicAdd32Barrier(amount, value); }
int32_t apple_os_atomic_increment32(volatile int32_t *value) { return OSAtomicIncrement32(value); }
int32_t apple_os_atomic_increment32_barrier(volatile int32_t *value) { return OSAtomicIncrement32Barrier(value); }
int32_t apple_os_atomic_decrement32(volatile int32_t *value) { return OSAtomicDecrement32(value); }
int32_t apple_os_atomic_decrement32_barrier(volatile int32_t *value) { return OSAtomicDecrement32Barrier(value); }
int32_t apple_os_atomic_or32(uint32_t mask, volatile uint32_t *value) { return OSAtomicOr32(mask, value); }
int32_t apple_os_atomic_or32_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicOr32Barrier(mask, value); }
int32_t apple_os_atomic_or32_orig(uint32_t mask, volatile uint32_t *value) { return OSAtomicOr32Orig(mask, value); }
int32_t apple_os_atomic_or32_orig_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicOr32OrigBarrier(mask, value); }
int32_t apple_os_atomic_and32(uint32_t mask, volatile uint32_t *value) { return OSAtomicAnd32(mask, value); }
int32_t apple_os_atomic_and32_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicAnd32Barrier(mask, value); }
int32_t apple_os_atomic_and32_orig(uint32_t mask, volatile uint32_t *value) { return OSAtomicAnd32Orig(mask, value); }
int32_t apple_os_atomic_and32_orig_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicAnd32OrigBarrier(mask, value); }
int32_t apple_os_atomic_xor32(uint32_t mask, volatile uint32_t *value) { return OSAtomicXor32(mask, value); }
int32_t apple_os_atomic_xor32_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicXor32Barrier(mask, value); }
int32_t apple_os_atomic_xor32_orig(uint32_t mask, volatile uint32_t *value) { return OSAtomicXor32Orig(mask, value); }
int32_t apple_os_atomic_xor32_orig_barrier(uint32_t mask, volatile uint32_t *value) { return OSAtomicXor32OrigBarrier(mask, value); }
bool apple_os_atomic_compare_and_swap32(int32_t old_value, int32_t new_value, volatile int32_t *value) { return OSAtomicCompareAndSwap32(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap32_barrier(int32_t old_value, int32_t new_value, volatile int32_t *value) { return OSAtomicCompareAndSwap32Barrier(old_value, new_value, value); }

int64_t apple_os_atomic_add64(int64_t amount, volatile int64_t *value) { return OSAtomicAdd64(amount, (volatile OSAtomic_int64_aligned64_t *)value); }
int64_t apple_os_atomic_add64_barrier(int64_t amount, volatile int64_t *value) { return OSAtomicAdd64Barrier(amount, (volatile OSAtomic_int64_aligned64_t *)value); }
int64_t apple_os_atomic_increment64(volatile int64_t *value) { return OSAtomicIncrement64((volatile OSAtomic_int64_aligned64_t *)value); }
int64_t apple_os_atomic_increment64_barrier(volatile int64_t *value) { return OSAtomicIncrement64Barrier((volatile OSAtomic_int64_aligned64_t *)value); }
int64_t apple_os_atomic_decrement64(volatile int64_t *value) { return OSAtomicDecrement64((volatile OSAtomic_int64_aligned64_t *)value); }
int64_t apple_os_atomic_decrement64_barrier(volatile int64_t *value) { return OSAtomicDecrement64Barrier((volatile OSAtomic_int64_aligned64_t *)value); }
bool apple_os_atomic_compare_and_swap64(int64_t old_value, int64_t new_value, volatile int64_t *value) { return OSAtomicCompareAndSwap64(old_value, new_value, (volatile OSAtomic_int64_aligned64_t *)value); }
bool apple_os_atomic_compare_and_swap64_barrier(int64_t old_value, int64_t new_value, volatile int64_t *value) { return OSAtomicCompareAndSwap64Barrier(old_value, new_value, (volatile OSAtomic_int64_aligned64_t *)value); }

bool apple_os_atomic_compare_and_swap_int(int old_value, int new_value, volatile int *value) { return OSAtomicCompareAndSwapInt(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap_int_barrier(int old_value, int new_value, volatile int *value) { return OSAtomicCompareAndSwapIntBarrier(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap_long(long old_value, long new_value, volatile long *value) { return OSAtomicCompareAndSwapLong(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap_long_barrier(long old_value, long new_value, volatile long *value) { return OSAtomicCompareAndSwapLongBarrier(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap_ptr(void *old_value, void *new_value, void *volatile *value) { return OSAtomicCompareAndSwapPtr(old_value, new_value, value); }
bool apple_os_atomic_compare_and_swap_ptr_barrier(void *old_value, void *new_value, void *volatile *value) { return OSAtomicCompareAndSwapPtrBarrier(old_value, new_value, value); }
bool apple_os_atomic_test_and_set(uint32_t bit, volatile void *address) { return OSAtomicTestAndSet(bit, address); }
bool apple_os_atomic_test_and_set_barrier(uint32_t bit, volatile void *address) { return OSAtomicTestAndSetBarrier(bit, address); }
bool apple_os_atomic_test_and_clear(uint32_t bit, volatile void *address) { return OSAtomicTestAndClear(bit, address); }
bool apple_os_atomic_test_and_clear_barrier(uint32_t bit, volatile void *address) { return OSAtomicTestAndClearBarrier(bit, address); }
