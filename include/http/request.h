#pragma once

#include "llhttp.h"
#include "str.h"
#include "headers.h"

typedef struct http_raw_request {
    llhttp_t parser;
    str_t url;
    header_map_t headers;
} http_raw_request_t;

#define request_for_parser(PARSER) \
    __containerof(PARSER, http_raw_request_t, parser);
