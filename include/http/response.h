#pragma once

#include "lwip/tcp.h"
#include "pico/platform.h"
#include "chunked_str.h"

typedef struct http_raw_response {
    chunked_str_t data;
    size_t sent;
} http_raw_response_t;

inline static size_t response_remaining_len(http_raw_response_t *transaction) {
    return chunked_str_total_length(&transaction->data) - transaction->sent;
}

err_t response_send(http_raw_response_t *response,
                    struct tcp_pcb *client_pcb);
