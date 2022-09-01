#pragma once

#include <stdio.h>

#define STRINGIFY(str) #str
#define LOG(LEVEL, MESSAGE) \
    puts(__FILE__ ":" STRINGIFY(__LINE__) " [" #LEVEL "] " #MESSAGE)

#define LOG_LEVEL_TRACE 5
#define LOG_LEVEL_DEBUG 4
#define LOG_LEVEL_INFO 3
#define LOG_LEVEL_WARN 2
#define LOG_LEVEL_ERROR 1
#define LOG_LEVEL_NONE 0

#if LOG_LEVEL >= LOG_LEVEL_TRACE
#define TRACE(MESSAGE) LOG("TRACE", MESSAGE)
#else
#define TRACE(MESSAGE)
#endif

#if LOG_LEVEL >= LOG_LEVEL_DEBUG
#define DEBUG(MESSAGE) LOG("DEBUG", MESSAGE)
#else
#define DEBUG(MESSAGE)
#endif

#if LOG_LEVEL >= LOG_LEVEL_INFO
#define INFO(MESSAGE) LOG("INFO ", MESSAGE)
#else
#define INFO(MESSAGE)
#endif

#if LOG_LEVEL >= LOG_LEVEL_WARN
#define WARN(MESSAGE) LOG("WARN ", MESSAGE)
#else
#define WARN(MESSAGE)
#endif

#if LOG_LEVEL >= LOG_LEVEL_ERROR
#define ERROR(MESSAGE) LOG("ERROR", MESSAGE)
#else
#define ERROR(MESSAGE)
#endif
