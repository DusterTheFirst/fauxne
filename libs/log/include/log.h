#pragma once

#include "ansi.h"
#include "pico/stdio.h"
#include "pico/time.h"
#include <stdarg.h>
#include <stdio.h>

static inline void log_preamble(const char *const file,
                                int line,
                                const char *const level) {
    const uint32_t ms = to_ms_since_boot(get_absolute_time());
    const uint32_t whole_ms = ms % 1000;
    const uint32_t whole_sec = ms / 1000 % 60;
    const uint32_t whole_min = ms / (1000 * 60) & 60;

    printf(ANSI_FOREGROUND_WHITE
           "%.02ld:%.02ld.%.03ld " ANSI_FOREGROUND_GRAY "%s:%d"
           " [" ANSI_SGR_RESET "%s" ANSI_FOREGROUND_GRAY
           "] " ANSI_FOREGROUND_BRIGHT_WHITE,
           whole_min, whole_sec, whole_ms,
           file, line, level);
}

#define LOG_PREAMBLE(LEVEL) log_preamble(__FILE__, __LINE__, LEVEL)

#define LOG_LEVEL_TRACE 5
#define LOG_LEVEL_DEBUG 4
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_NONE 0

#define LOG_CAN_TRACE LOG_LEVEL >= LOG_LEVEL_TRACE
#define LOG_CAN_DEBUG LOG_LEVEL >= LOG_LEVEL_DEBUG
#define LOG_CAN_INFO LOG_LEVEL >= LOG_LEVEL_INFO
#define LOG_CAN_WARN LOG_LEVEL >= LOG_LEVEL_WARN
#define LOG_CAN_ERROR LOG_LEVEL >= LOG_LEVEL_ERROR

#if LOG_CAN_TRACE

#define TRACE_PREAMBLE LOG_PREAMBLE(ANSI_FOREGROUND_MAGENTA "TRACE")
#define TRACE(MESSAGE, ...) \
    TRACE_PREAMBLE;         \
    printf(MESSAGE "\n" __VA_OPT__(, __VA_ARGS__))

#else
#define TRACE_PREAMBLE
#define TRACE(MESSAGE, ...)
#endif

#if LOG_CAN_DEBUG
#define DEBUG_PREAMBLE LOG_PREAMBLE(ANSI_FOREGROUND_CYAN "DEBUG")
#define DEBUG(MESSAGE, ...) \
    DEBUG_PREAMBLE;         \
    printf(MESSAGE "\n" __VA_OPT__(, __VA_ARGS__))

#define DBG(FMT, VAR)                      \
    DEBUG(ANSI_FOREGROUND_GRAY #VAR        \
          " = " ANSI_FOREGROUND_WHITE      \
              FMT ANSI_FOREGROUND_GRAY "", \
          VAR)
#else
#define DEBUG_PREAMBLE
#define DEBUG(MESSAGE, ...)
#define DBG(FMT, VAR)
#endif

#if LOG_CAN_INFO
#define INFO_PREAMBLE LOG_PREAMBLE(ANSI_FOREGROUND_BLUE "INFO ")
#define INFO(MESSAGE, ...) \
    INFO_PREAMBLE;         \
    printf(MESSAGE "\n" __VA_OPT__(, __VA_ARGS__))

#else
#define INFO_PREAMBLE
#define INFO(MESSAGE, ...)
#endif

#if LOG_CAN_WARN
#define WARN_PREAMBLE LOG_PREAMBLE(ANSI_FOREGROUND_YELLOW "WARN ")
#define WARN(MESSAGE, ...) \
    WARN_PREAMBLE;         \
    printf(MESSAGE "\n" __VA_OPT__(, __VA_ARGS__))

#else
#define WARN_PREAMBLE
#define WARN(MESSAGE, ...)
#endif

#if LOG_CAN_ERROR
#define ERROR_PREAMBLE LOG_PREAMBLE(ANSI_FOREGROUND_RED "ERROR")
#define ERROR(MESSAGE, ...) \
    ERROR_PREAMBLE;         \
    printf(MESSAGE "\n" __VA_OPT__(, __VA_ARGS__))

#else
#define ERROR_PREAMBLE
#define ERROR(MESSAGE, ...)
#endif
