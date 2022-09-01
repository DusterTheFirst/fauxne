#pragma once

#include <stdio.h>

#define S_INTERNAL(x) #x
#define STRINGIFY(x) S_INTERNAL(x)

#define __FILE_LINE__ __FILE__ ":" STRINGIFY(__LINE__)

#define LOG(LEVEL, MESSAGE, ...)                                   \
    {                                                              \
        const uint32_t ms = to_ms_since_boot(get_absolute_time()); \
        printf("%ld.%.03ld " __FILE_LINE__                         \
               " [" LEVEL "] " MESSAGE "\n",                       \
               ms / 1000, ms % 1000 __VA_OPT__(, __VA_ARGS__));    \
    }

#define LOG_LEVEL_TRACE 5
#define LOG_LEVEL_DEBUG 4
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_NONE 0

#if LOG_LEVEL >= LOG_LEVEL_TRACE
#define TRACE(...) LOG("TRACE", __VA_ARGS__)
#else
#define TRACE(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_DEBUG
#define DEBUG(...) LOG("DEBUG", __VA_ARGS__)
#define DBG_str(VAR) LOG("DEBUG", #VAR " = %s", VAR)
#else
#define DEBUG(...)
#define DBG_str(VAR)
#endif

#if LOG_LEVEL >= LOG_LEVEL_INFO
#define INFO(...) LOG("INFO ", __VA_ARGS__)
#else
#define INFO(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_WARN
#define WARN(...) LOG("WARN ", __VA_ARGS__)
#else
#define WARN(...)
#endif

#if LOG_LEVEL >= LOG_LEVEL_ERROR
#define ERROR(...) LOG("ERROR", __VA_ARGS__)
#else
#define ERROR(...)
#endif
