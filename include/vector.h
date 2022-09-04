#pragma once

#include <stdlib.h>

#define DEFINE_VECTOR(TYPE, NAME)                                          \
    typedef struct NAME {                                                  \
        TYPE *buffer;                                                      \
        size_t capacity;                                                   \
        size_t length;                                                     \
    } NAME##_t;                                                            \
                                                                           \
    static inline NAME##_t NAME##_new(void) {                              \
        return ((NAME##_t){                                                \
            .buffer = NULL,                                                \
            .capacity = 0,                                                 \
            .length = 0,                                                   \
        });                                                                \
    }                                                                      \
                                                                           \
    static inline void NAME##_clear(NAME##_t *vector) {                    \
        free(vector->buffer);                                              \
                                                                           \
        *vector = NAME##_new();                                            \
    }                                                                      \
                                                                           \
    static inline void NAME##_free(NAME##_t *vector) {                     \
        NAME##_clear(vector);                                              \
    }                                                                      \
                                                                           \
    static inline TYPE *NAME##_get(NAME##_t *vector, size_t index) {       \
        if (index >= vector->length) {                                     \
            return NULL;                                                   \
        }                                                                  \
                                                                           \
        return &vector->buffer[index];                                     \
    }                                                                      \
                                                                           \
    static inline TYPE *NAME##_last(NAME##_t *vector) {                    \
        if (vector->length == 0) {                                         \
            return NULL;                                                   \
        }                                                                  \
                                                                           \
        return &vector->buffer[vector->length - 1];                        \
    }                                                                      \
                                                                           \
    static inline TYPE *NAME##_push(NAME##_t *vector, TYPE value) {        \
        if (vector == NULL) {                                              \
            return NULL;                                                   \
        }                                                                  \
                                                                           \
        if (vector->length == vector->capacity) {                          \
            if (vector->buffer == NULL) {                                  \
                vector->capacity = 2;                                      \
                vector->buffer = malloc(sizeof(TYPE) * vector->capacity);  \
            } else {                                                       \
                vector->capacity *= 2;                                     \
                vector->buffer = realloc(vector->buffer,                   \
                                         sizeof(TYPE) * vector->capacity); \
            }                                                              \
        }                                                                  \
                                                                           \
        TYPE *new_item = &vector->buffer[vector->length];                  \
        vector->length += 1;                                               \
                                                                           \
        *new_item = value;                                                 \
        return new_item;                                                   \
    }                                                                      \
                                                                           \
    static inline void NAME##_shrink_to_fit(NAME##_t *vector) {            \
        if (vector == NULL) {                                              \
            return;                                                        \
        }                                                                  \
                                                                           \
        vector->capacity = vector->length;                                 \
        vector->buffer = realloc(vector->buffer,                           \
                                 sizeof(TYPE) * vector->capacity);         \
    }
