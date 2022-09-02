#pragma once

#define TRY(CODE, MESSAGE)                     \
    {                                          \
        err_t __err = (CODE);                    \
                                               \
        if (__err != ERR_OK) {                   \
            ERROR(MESSAGE " (error %d)", __err); \
            return __err;                        \
        }                                      \
    }
