#include "error.h"
#include "http.h"
#include "http/request.h"
#include "http/response.h"
#include "log.h"
#include "lwip/tcp.h"

err_t server_sent(http_conn_state_t *state, struct tcp_pcb *client_pcb,
                  u16_t len) {
    DEBUG("TCP server sent %u bytes", len);

    state->response.sent += len;

    if (state->response.sent == state->response.length) {
        TRACE("sent all that needs to be sent, closing connection");

        TCP_TRY(tcp_close(client_pcb), "Encountered error trying to close pcb");

        state->response.sent = 0;
    }

    return ERR_OK;
}

err_t server_recv(http_conn_state_t *state,
                  struct tcp_pcb *client_pcb,
                  struct pbuf *p,
                  err_t err) {
    TCP_TRY(err, "Encountered error receiving data");

    llhttp_errno_t llhttp_error =
        llhttp_execute(&state->request.parser, p->payload, p->len);

    DBG("%d", state->request.parser.http_major);
    DBG("%d", state->request.parser.http_minor);
    DBG("%lld", state->request.parser.content_length);
    DBG("%p", state->request.parser.data);
    DBG("%ld", state->request.parser.error);
    DBG("%d", state->request.parser.finish);
    DBG("%d", state->request.parser.flags);
    DBG("%d", state->request.parser.header_state);
    DBG("%d", state->request.parser.lenient_flags);
    DBG("%d", state->request.parser.method);
    DBG("%s", state->request.parser.reason);
    DBG("%d", state->request.parser.status_code);
    DBG("%d", state->request.parser.type);
    DBG("%d", state->request.parser.upgrade);

    DEBUG("llhttp returned %d: %s", llhttp_error,
          llhttp_errno_name(llhttp_error))

    DEBUG("Received %d bytes", p->len);
    tcp_recved(client_pcb, p->len);
    pbuf_free(p);

    return ERR_OK;
}

err_t server_poll(http_conn_state_t *state,
                  struct tcp_pcb *client_pcb) {
    TCP_TRY(response_send(&state->response, client_pcb),
            "Failed to send http response");

    return ERR_OK;
}

void server_err(__unused http_conn_state_t *state, err_t err) {
    TCP_ERROR(err, "TCP server error");
}

int __test_3(__unused llhttp_t *parser, const char *at, size_t length) {
    if (at == NULL) {
        return -1;
    }

    DEBUG("%.*s", length, at);

    return 0;
}

int __test_1(__unused llhttp_t *parser) {
    DEBUG("complete");

    return 0;
}

err_t server_accept(
    __unused http_server_t *server,
    struct tcp_pcb *client_pcb,
    err_t err) {
    TCP_TRY(err, "tcp server encountered an error accepting a connection");

    if (client_pcb == NULL) {
        ERROR("accept handler was provided an null client_pcb");

        return ERR_VAL;
    }

    TRACE("tcp client connected from %d.%d.%d.%d:%d",
          ip4_addr1(&client_pcb->remote_ip), ip4_addr2(&client_pcb->remote_ip),
          ip4_addr3(&client_pcb->remote_ip), ip4_addr4(&client_pcb->remote_ip),
          client_pcb->remote_port);

    static const char http_response[] =
        "HTTP/1.1 200 OK\r\n"
        "Content-Length: 13\r\n"
        "Content-Type: text/html\r\n"
        // "Connection: keep-alive\r\n"
        // "Keep-Alive: timeout=5, max=1000\r\n"
        "\r\n"
        "<h1>piss</h1>";

    http_conn_state_t *conn_state = malloc(sizeof(http_conn_state_t));

    *conn_state = (http_conn_state_t){
        .request = (http_raw_request_t){
            .parser = {},
            .parser_settings = {
                // .on_body = __test_3,
                // .on_chunk_complete = __test_1,
                // .on_chunk_header = __test_1,
                // .on_header_field = __test_3,
                // .on_header_field_complete = __test_1,
                // .on_header_value = __test_3,
                // .on_header_value_complete = __test_1,
                // .on_headers_complete = __test_1,
                // .on_message_begin = __test_1,
                // .on_message_complete = __test_1,
                // .on_status = __test_3,
                // .on_status_complete = __test_1,
                // .on_url = __test_3,
                // .on_url_complete = __test_1,
            },
        },
        .response = (http_raw_response_t){
            .data = http_response,
            .length = sizeof(http_response) / sizeof(char),
            .sent = 0,
        },
    };

    llhttp_settings_t piss = {};

    llhttp_init(&conn_state->request.parser,
                HTTP_REQUEST,
                &piss);
                // &conn_state->request.parser_settings);

    tcp_arg(client_pcb, conn_state);
    tcp_sent(client_pcb, (tcp_sent_fn)server_sent);
    tcp_recv(client_pcb, (tcp_recv_fn)server_recv);

#define POLL_TIME_S 5
    tcp_poll(client_pcb, (tcp_poll_fn)server_poll, POLL_TIME_S * 2);
    tcp_err(client_pcb, (tcp_err_fn)server_err);

    TCP_TRY(response_send(&conn_state->response, client_pcb),
            "failed to send transaction");

    return ERR_OK;
}
