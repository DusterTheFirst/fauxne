#pragma once

#include "log.h"
#include "pico/stdlib.h"
#include <string.h>

typedef struct str {
    const char *ptr;
    size_t len;
} str_t;

#define str(CSTR)                \
    ((str_t){                    \
        .ptr = CSTR,             \
        .len = sizeof(CSTR) - 1, \
    })

static inline str_t str_from_raw(const char *ptr, size_t len) {
    return ((str_t){
        .ptr = ptr,
        .len = len,
    });
}

static inline str_t str_from_cstr(const char *ptr) {
    return str_from_raw(ptr, strlen(ptr));
}

static inline str_t str_slice_to_end(str_t *str, size_t start) {
    if (start >= str->len) {
        panic("attempted to index past end of slice");

        return ((str_t){
            .ptr = NULL,
            .len = 0,
        });
    }

    return str_from_raw(str->ptr + start, str->len - start);
}

#define DBG_str(STR) DEBUG(ANSI_FOREGROUND_GRAY #STR         \
                           " = \"" ANSI_FOREGROUND_WHITE     \
                           "%.*s" ANSI_FOREGROUND_GRAY "\"", \
                           STR.len, STR.ptr);
