#pragma once

#include "headers.h"
#include "llhttp.h"
#include "lwip/pbuf.h"
#include "str.h"

typedef struct http_raw_request {
    llhttp_t parser;
    struct pbuf *packets;

    chunked_str_t url;
    header_map_t headers;
} http_raw_request_t;

#define request_for_parser(PARSER) \
    __containerof(PARSER, http_raw_request_t, parser);
