#pragma once

#include "log.h"
#include "lwip/err.h"

const char *tcp_error_name(err_t code);

#define TCP_TRY(CODE, MESSAGE, ...)                              \
    {                                                            \
        const err_t __err = (CODE);                              \
                                                                 \
        if (__err != ERR_OK) {                                   \
            TCP_ERROR(__err, MESSAGE __VA_OPT__(, __VA_ARGS__)); \
            return __err;                                        \
        }                                                        \
    }

#define TCP_ERROR(_ERROR, MESSAGE, ...)        \
    ERROR_PREAMBLE;                            \
    printf(MESSAGE __VA_OPT__(, __VA_ARGS__)); \
    printf(": %s (error %d)\n", tcp_error_name(_ERROR), _ERROR)

#define LLHTTP_ERROR(_ERROR, MESSAGE, ...)     \
    ERROR_PREAMBLE;                            \
    printf(MESSAGE __VA_OPT__(, __VA_ARGS__)); \
    printf(": %s (error %d)", llhttp_errno_name(_ERROR), _ERROR)
