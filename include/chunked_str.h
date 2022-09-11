#pragma once

#include "str.h"
#include "string.h"
#include "vector.h"

DEFINE_VECTOR(str_t, chunked_str)

static inline size_t chunked_str_total_length(const chunked_str_t
                                                  *const chunked) {
    size_t length = 0;

    for (size_t i = 0; i < chunked->length; i++) {
        length += chunked->buffer[i].len;
    }

    return length;
}

static inline bool chunked_str_eq(const chunked_str_t *const chunked,
                                  const str_t str) {
    size_t cursor = 0;

    for (size_t i = 0; i < chunked->length; i++) {
        size_t chunk_len = chunked->buffer[i].len;
        size_t str_remaining = str.len - cursor;

        if (chunk_len != str_remaining) {
            return false;
        }

        uint32_t result = strncmp(chunked->buffer[i].ptr,
                                  &str.ptr[cursor], chunk_len);

        if (result != 0) {
            return false;
        }

        cursor += chunked->buffer[i].len;
    }

    return true;
}

static inline void chunked_str_printf(chunked_str_t *chunked) {
    for (size_t i = 0; i < chunked->length; i++) {
        str_t *str = &chunked->buffer[i];

        printf("%.*s", str->len, str->ptr);
    }
}
