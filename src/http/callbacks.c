#include "chunked_str.h"
#include "error.h"
#include "http.h"
#include "http/parse.h"
#include "http/request.h"
#include "http/response.h"
#include "log.h"
#include "lwip/tcp.h"

err_t callback_sent(http_conn_state_t *state,
                    __unused struct tcp_pcb *client_pcb,
                    u16_t len) {
    DEBUG("TCP server sent %u bytes", len);

    state->response.sent += len;

    if (state->response.sent ==
        chunked_str_total_length(&state->response.data)) {
        TRACE("sent all that needs to be sent, closing connection");

        TCP_TRY(tcp_close(client_pcb), "Encountered error trying to close pcb");

        state->response.sent = 0;
    }

    return ERR_OK;
}

err_t finish_http(http_conn_state_t *state,
                  struct tcp_pcb *client_pcb) {
    {
        llhttp_errno_t llhttp_error = llhttp_finish(&state->request.parser);
        if (llhttp_error != HPE_OK) {
            LLHTTP_ERROR(llhttp_error, "llhttp is in an error %s",
                         llhttp_get_error_reason(&state->request.parser));

            return ERR_VAL;
        }
    }

    // Finish code:
    {
        header_map_debug(&state->request.headers);

        // TODO: unscuff this so it actually works with DBG/DEBUG
        DEBUG_PREAMBLE;
        printf("url: ");
        chunked_str_printf(&state->request.url);
        printf("\n");

        // TODO: ensure we do not accidentally send the response until ready
        state->response.data = http_server_respond(&state->request);

        TCP_TRY(response_send(&state->response, client_pcb),
                "failed to send transaction");
    }

    return ERR_OK;
}

err_t callback_receive(http_conn_state_t *state,
                       struct tcp_pcb *client_pcb,
                       struct pbuf *pbuf,
                       err_t err) {
    TCP_TRY(err, "Encountered error receiving data");

    if (pbuf == NULL) {
        INFO("connection closed");

        tcp_close(client_pcb);
        pbuf_free(state->request.packets);
        chunked_str_free(&state->response.data);

        return ERR_OK;
    }

    TRACE("Received %d bytes (%d)", pbuf->tot_len, pbuf->len);
    // INFO("%.*s", pbuf->len, (char *)(pbuf->payload));

    for (struct pbuf *buffer = pbuf; buffer != NULL; buffer = buffer->next) {
        llhttp_errno_t llhttp_error =
            llhttp_execute(&state->request.parser, pbuf->payload, pbuf->len);

        if (llhttp_error != HPE_OK) {
            LLHTTP_ERROR(llhttp_error, "llhttp is in an error %s",
                         llhttp_get_error_reason(&state->request.parser));

            return ERR_VAL;
        }
    }

    // DBG("%d", state->request.parser.http_major);
    // DBG("%d", state->request.parser.http_minor);
    // DBG("%lld", state->request.parser.content_length);
    // DBG("%p", state->request.parser.data);
    // DBG("%ld", state->request.parser.error);
    // DBG("%d", state->request.parser.finish);
    // DBG("%d", state->request.parser.flags);
    // DBG("%d", state->request.parser.header_state);
    // DBG("%d", state->request.parser.lenient_flags);
    // DBG("%d", state->request.parser.method);
    // DBG("%s", state->request.parser.reason);
    // DBG("%d", state->request.parser.status_code);
    // DBG("%d", state->request.parser.type);
    // DBG("%d", state->request.parser.upgrade);

    tcp_recved(client_pcb, pbuf->len);

    // Save a pointer to the packet buffer so we can free it later
    if (state->request.packets == NULL) {
        state->request.packets = pbuf;
    } else {
        pbuf_chain(state->request.packets, pbuf);
    }

    // Finished with http parsing
    if (state->request.parser.finish == HTTP_FINISH_SAFE) {
        return finish_http(state, client_pcb);
    }

    return ERR_OK;
}

err_t callback_poll(__unused http_conn_state_t *state,
                    __unused struct tcp_pcb *client_pcb) {
    // TODO: continue sending response if response sending started

    return ERR_OK;
}

void callback_error(__unused http_conn_state_t *state, err_t err) {
    TCP_ERROR(err, "TCP server error");
}

err_t callback_accept(
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

    http_conn_state_t *conn_state = malloc(sizeof(http_conn_state_t));

    *conn_state = (http_conn_state_t){
        .request = (http_raw_request_t){
            .parser = {},
            .url = chunked_str_new(),
            .packets = NULL,
            .headers = header_map_new(),
        },
        .response = (http_raw_response_t){
            .data = {},
            .sent = 0,
        },
    };

    llhttp_init(&conn_state->request.parser, HTTP_REQUEST, &parser_callbacks);

    tcp_arg(client_pcb, conn_state);
    tcp_sent(client_pcb, (tcp_sent_fn)callback_sent);
    tcp_recv(client_pcb, (tcp_recv_fn)callback_receive);

#define POLL_TIME_S 5
    tcp_poll(client_pcb, (tcp_poll_fn)callback_poll, POLL_TIME_S * 2);
    tcp_err(client_pcb, (tcp_err_fn)callback_error);

    return ERR_OK;
}
