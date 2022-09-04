#pragma once

#include "llhttp.h"

typedef struct http_raw_request {
    llhttp_t parser;
    llhttp_settings_t parser_settings;
} http_raw_request_t;
