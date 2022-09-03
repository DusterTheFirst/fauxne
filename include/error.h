#pragma once

#include "log.h"

#define TCP_TRY(CODE, MESSAGE, ...)                             \
    {                                                           \
        const err_t __err = (CODE);                             \
                                                                \
        if (__err != ERR_OK) {                                  \
            TCP_ERROR(__err, MESSAGE __VA_OPT__(, __VA_ARGS__)) \
            return __err;                                       \
        }                                                       \
    }

#define TCP_ERROR(_ERROR, MESSAGE, ...)                        \
    ERROR(MESSAGE ": %s (error %d)" __VA_OPT__(, __VA_ARGS__), \
          tcp_error(_ERROR), _ERROR);

#include "lwip/err.h"

inline char *tcp_error(err_t code) {
    switch (code) {
        case ERR_OK:
            return "No error, everything OK";
        case ERR_MEM:
            return "Out of memory error";
        case ERR_BUF:
            return "Buffer error";
        case ERR_TIMEOUT:
            return "Timeout";
        case ERR_RTE:
            return "Routing problem";
        case ERR_INPROGRESS:
            return "Operation in progress";
        case ERR_VAL:
            return "Illegal value";
        case ERR_WOULDBLOCK:
            return "Operation would block";
        case ERR_USE:
            return "Address in use";
        case ERR_ALREADY:
            return "Already connecting";
        case ERR_ISCONN:
            return "Connection already established";
        case ERR_CONN:
            return "Not connected";
        case ERR_IF:
            return "Low-level netif error";
        case ERR_ABRT:
            return "Connection aborted";
        case ERR_RST:
            return "Connection reset";
        case ERR_CLSD:
            return "Connection closed";
        case ERR_ARG:
            return "Illegal argument";
        default:
            return "UNKNOWN ERROR";
    }
}
