#pragma once

#include "error.h"
#include "log.h"
#include "lwip/tcp.h"

err_t server_sent(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb,
                  u16_t len);

err_t server_recv(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb,
                  struct pbuf *p,
                  err_t err);

err_t server_poll(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb);

void server_err(http_conn_state_t *arg,
                err_t err);

err_t server_accept(http_conn_state_t *server,
                    struct tcp_pcb *client_pcb,
                    err_t err);
