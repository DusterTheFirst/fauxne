#pragma once

#include "error.h"
#include "log.h"
#include "lwip/tcp.h"

err_t callback_sent(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb,
                  u16_t len);

err_t callback_receive(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb,
                  struct pbuf *p,
                  err_t err);

err_t callback_poll(http_conn_state_t *server,
                  struct tcp_pcb *client_pcb);

void callback_error(http_conn_state_t *arg,
                err_t err);

err_t callback_accept(http_conn_state_t *server,
                    struct tcp_pcb *client_pcb,
                    err_t err);
