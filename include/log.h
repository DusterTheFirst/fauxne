#pragma once

#include "ansi.h"
#include "pico/stdio.h"
#include "pico/time.h"
#include <stdio.h>

#define LOG(LEVEL, MESSAGE, ...)                                           \
    {                                                                      \
        const uint32_t ms = to_ms_since_boot(get_absolute_time());         \
        const uint32_t whole_ms = ms % 1000;                               \
        const uint32_t whole_sec = ms / 1000 % 60;                         \
        const uint32_t whole_min = ms / (1000 * 60) & 60;                  \
        printf(ANSI_FOREGROUND_WHITE                                       \
               "%.02ld:%.02ld.%.03ld " ANSI_FOREGROUND_GRAY __FILE__ ":%d" \
               " [" ANSI_RESET LEVEL ANSI_FOREGROUND_GRAY                  \
               "] " ANSI_FOREGROUND_BRIGHT_WHITE MESSAGE "\n",             \
               whole_min, whole_sec, whole_ms,                             \
               __LINE__ __VA_OPT__(, __VA_ARGS__));                        \
    }

#define LOG_LEVEL_TRACE 5
#define LOG_LEVEL_DEBUG 4
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_NONE 0

#if LOG_LEVEL >= LOG_LEVEL_TRACE

#define TRACE(...) LOG(ANSI_FOREGROUND_MAGENTA "TRACE", __VA_ARGS__)

#else
#define TRACE(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_DEBUG
#define DEBUG(...) LOG(ANSI_FOREGROUND_CYAN "DEBUG", __VA_ARGS__)
#define DBG(FMT, VAR)                        \
    DEBUG(ANSI_FOREGROUND_GRAY #VAR          \
          " = " ANSI_FOREGROUND_WHITE      \
              FMT ANSI_FOREGROUND_GRAY "", \
          VAR)
#else
#define DEBUG(...)
#define DBG_str(VAR)
#endif

#if LOG_LEVEL >= LOG_LEVEL_INFO
#define INFO(...) LOG(ANSI_FOREGROUND_BLUE "INFO ", __VA_ARGS__)
#else
#define INFO(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_WARN
#define WARN(...) LOG(ANSI_FOREGROUND_YELLOW "WARN ", __VA_ARGS__)
#else
#define WARN(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_ERROR
#define ERROR(...) LOG(ANSI_FOREGROUND_RED "ERROR", __VA_ARGS__)
#else
#define ERROR(...)
#endif
