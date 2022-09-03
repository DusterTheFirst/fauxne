#pragma once

#include "http/request.h"
#include "http/response.h"
#include "llhttp.h"
#include "log.h"
#include "lwip/tcp.h"

typedef struct http_server {
    struct tcp_pcb *server_pcb;
} http_server_t;

err_t http_server_init(ip4_addr_t *ip, uint16_t port,
                       http_server_t *server);
err_t http_server_close(http_server_t *server);

inline static ip4_addr_t const *http_server_local_ip(http_server_t *server) {
    return &server->server_pcb->local_ip;
}

inline static uint16_t http_server_local_port(http_server_t *server) {
    return server->server_pcb->local_port;
}

typedef struct http_conn_state {
    http_raw_response_t response;
    http_raw_request_t request;
} http_conn_state_t;
