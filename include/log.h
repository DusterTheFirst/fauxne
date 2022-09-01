#pragma once

#include <stdio.h>

#define STRINGIFY(str) #str
#define LOG(LEVEL, MESSAGE, ...) \
    printf(__FILE__ ":" STRINGIFY(__LINE__) " [" LEVEL "] " MESSAGE "\n" __VA_OPT__(, __VA_ARGS__)) // NOLINT

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
