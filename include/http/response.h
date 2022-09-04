#pragma once

#include "lwip/tcp.h"
#include "pico/platform.h"
#include "str.h"

typedef struct http_raw_response {
    str_t response_text;
    size_t sent;
} http_raw_response_t;

inline static size_t response_remaining_len(http_raw_response_t *transaction) {
    return transaction->response_text.len - transaction->sent;
}

inline static void const *response_remaining_data(
    http_raw_response_t *transaction) {
    return transaction->response_text.ptr +
           MIN(transaction->sent, transaction->response_text.len);
}

err_t response_send(http_raw_response_t *response,
                    struct tcp_pcb *client_pcb);
