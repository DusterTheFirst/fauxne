#pragma once

#include "str.h"
#include "vector.h"

DEFINE_VECTOR(str_t, chunked_str)

static inline size_t chunked_str_total_length(chunked_str_t *chunked) {
    size_t length = 0;

    for (size_t i = 0; i < chunked->length; i++) {
        length += chunked->buffer[i].len;
    }

    return length;
}

static inline void chunked_str_printf(chunked_str_t *chunked) {
    for (size_t i = 0; i < chunked->length; i++) {
        str_t *str = &chunked->buffer[i];

        printf("%.*s", str->len, str->ptr);
    }
}
