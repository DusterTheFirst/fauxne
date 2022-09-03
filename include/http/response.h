#pragma once

#include "lwip/tcp.h"
#include "pico/platform.h"

typedef struct http_raw_response {
    void const *data;
    size_t length;
    size_t sent;
} http_raw_response_t;

inline static size_t response_remaining_len(http_raw_response_t *transaction) {
    return transaction->length - transaction->sent;
}

inline static void const *response_remaining_data(
    http_raw_response_t *transaction) {
    return transaction->data + MIN(transaction->sent, transaction->length);
}

err_t response_send(http_raw_response_t *response,
                    struct tcp_pcb *client_pcb);
