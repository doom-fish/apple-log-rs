#!/bin/bash
SDK=$(xcrun --sdk macosx --show-sdk-path)

# Function to check if a symbol exists and get its availability info
check_symbol() {
    local sym=$1
    local header=$2
    local filepath="$SDK/$header"
    
    if grep -q "\\b$sym\\b" "$filepath"; then
        # Get the availability attributes for this symbol
        local context=$(grep -B 3 "\\b$sym\\b" "$filepath" | grep -E "API_|@available|__OSX" | tail -1)
        echo "✓ $sym | $header | $context"
    else
        echo "✗ $sym | $header | NOT FOUND"
    fi
}

# Check core symbols
check_symbol "os_log_t" "usr/include/os/log.h"
check_symbol "os_log_type_t" "usr/include/os/log.h"
check_symbol "os_log_create" "usr/include/os/log.h"
check_symbol "os_log_type_enabled" "usr/include/os/log.h"
check_symbol "OS_LOG_DEFAULT" "usr/include/os/log.h"
check_symbol "OS_LOG_TYPE_DEBUG" "usr/include/os/log.h"
check_symbol "os_log_debug" "usr/include/os/log.h"

check_symbol "os_signpost_id_t" "usr/include/os/signpost.h"
check_symbol "os_signpost_id_generate" "usr/include/os/signpost.h"
check_symbol "os_signpost_enabled" "usr/include/os/signpost.h"

check_symbol "os_activity_t" "usr/include/os/activity.h"
check_symbol "os_activity_flag_t" "usr/include/os/activity.h"
check_symbol "os_activity_create" "usr/include/os/activity.h"
check_symbol "os_activity_apply" "usr/include/os/activity.h"
