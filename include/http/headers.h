#pragma once

#include "chunked_str.h"
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

static inline void header_map_debug(header_map_t *map) {
    DEBUG("header_map [");
    for (size_t header = 0; header < map->length; header += 1) {
        header_name_value_t *name_value = &map->buffer[header];

        chunked_str_printf(&name_value->header_name.text);
        printf(":\n");

        header_text_vector_t *header_values = &name_value->header_values;
        for (size_t value = 0; value < header_values->length; value += 1) {
            printf("    - \"");
            chunked_str_printf(&header_values->buffer[value].text);
            printf("\"\n");
        }
        putchar('\n');
    }
    printf("] header_map\n");
}
