#pragma once

#include "str.h"
#include "vector.h"

typedef struct header_text {
    chunked_str_t text;
    bool complete;
} header_text_t;

static inline header_text_t header_text_new(void) {
    return ((header_text_t){
        .complete = false,
        .text = chunked_str_new(),
    });
}

DEFINE_VECTOR(header_text_t, header_text_vector)

typedef struct header_name_value {
    header_text_t header_name;
    header_text_vector_t header_values;
} header_name_value_t;

static inline header_name_value_t header_name_value_new(void) {
    return ((header_name_value_t){
        .header_name = header_text_new(),
        .header_values = header_text_vector_new(),
    });
}

DEFINE_VECTOR(header_name_value_t, header_map)

// static inline void str_vector_debug(str_vector_t *vector) {
//     DEBUG("str_vector [");
//     for (size_t i = 0; i < vector->length; i++) {
//         str_t *str = &vector->buffer[i];

//         DEBUG("%d: %.*s", i, str->len, str->ptr);
//     }
//     printf("\n] str_vector\n");
// }
