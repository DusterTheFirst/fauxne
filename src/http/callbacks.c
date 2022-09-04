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

    if (state->response.sent == state->response.response_text.len) {
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
        printf("url: ");
        chunked_str_printf(&state->request.url);
        printf("\n");

        TCP_TRY(response_send(&state->response, client_pcb),
                "failed to send transaction");
    }

    // TODO: is this the right time?
    pbuf_free(state->request.packets);

    return ERR_OK;
}

err_t server_recv(http_conn_state_t *state,
                  struct tcp_pcb *client_pcb,
                  struct pbuf *pbuf,
                  err_t err) {
    TCP_TRY(err, "Encountered error receiving data");

    if (pbuf == NULL) {
        INFO("connection closed");

        return ERR_OK;
    }

    TRACE("Received %d bytes (%d)", pbuf->tot_len, pbuf->len);

    DBG("%d", pbuf_clen(pbuf));
    INFO("%.*s", pbuf->len, (char *)(pbuf->payload));

    for (struct pbuf *buffer = pbuf; buffer != NULL; buffer = buffer->next) {
        DBG("%p", buffer);

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

err_t server_poll(__unused http_conn_state_t *state,
                  __unused struct tcp_pcb *client_pcb) {
    // TODO: continue sending response if response sending started

    return ERR_OK;
}

void server_err(__unused http_conn_state_t *state, err_t err) {
    TCP_ERROR(err, "TCP server error");
}

static int on_url(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    chunked_str_push(&request->url, str_from_raw(at, length));

    return 0;
}

static int on_url_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    // Shrink vector to consume just the amount needed
    // TODO: may be using more cycles to realloc than is worth the spaces
    chunked_str_shrink_to_fit(&request->url);

    return 0;
}

static int on_header_field(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_field");

    str_t header_field = str_from_raw(at, length);
    DBG_str(header_field);

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL ||
        header_name_value->header_name.complete == true) {
        header_name_value =
            header_map_push(&request->headers, header_name_value_new());
    }

    header_text_t *header_text = &header_name_value->header_name;

    chunked_str_push(&header_text->text, header_field);

    return 0;
}

int on_header_field_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_field_complete");

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        return ERR_ABRT;
    }

    header_name_value->header_name.complete = true;

    // Shrink vector to consume just the amount needed
    chunked_str_shrink_to_fit(&header_name_value->header_name.text);

    return 0;
}

static int on_header_value(llhttp_t *parser, const char *at, size_t length) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_value");

    str_t header_value = str_from_raw(at, length);
    DBG_str(header_value);

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        header_name_value =
            header_map_push(&request->headers, header_name_value_new());
    }

    header_text_t *header_text =
        header_text_vector_last(&header_name_value->header_values);
    if (header_text == NULL) {
        header_text =
            header_text_vector_push(&header_name_value->header_values,
                                    header_text_new());
    }

    chunked_str_push(&header_text->text, header_value);

    return 0;
}

int on_header_value_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_header_value_complete");

    header_name_value_t *header_name_value = header_map_last(&request->headers);
    if (header_name_value == NULL) {
        return ERR_ABRT;
    }

    header_text_t *header_text =
        header_text_vector_last(&header_name_value->header_values);

    if (header_text == NULL) {
        return ERR_ABRT;
    }

    header_text->complete = true;

    // Shrink vector to consume just the amount needed
    chunked_str_shrink_to_fit(&header_text->text);
    header_text_vector_shrink_to_fit(&header_name_value->header_values);

    return 0;
}

int on_headers_complete(llhttp_t *parser) {
    http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_headers_complete");

    // Shrink vector to consume just the amount needed
    header_map_shrink_to_fit(&request->headers);

    return 0;
}

int on_chunk_header(llhttp_t *parser) {
    __unused http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_chunk_header");

    return 0;
}

int on_chunk_complete(llhttp_t *parser) {
    __unused http_raw_request_t *request = request_for_parser(parser);

    TRACE("on_chunk_complete");

    return 0;
}

// int __test_3(__unused llhttp_t *parser, const char *at, size_t length) {
//     http_raw_request_t *request = request_for_parser(parser);

//     return 0;
// }

// int __test_1(__unused llhttp_t *parser) {
//     http_raw_request_t *request = request_for_parser(parser);

//     return 0;
// }

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

    static const str_t http_response = str(
        "HTTP/1.1 200 OK\r\n"
        "Content-Length: 13\r\n"
        "Content-Type: text/html\r\n"
        // "Connection: keep-alive\r\n"
        // "Keep-Alive: timeout=5, max=1000\r\n"
        "\r\n"
        "<h1>piss</h1>");

    http_conn_state_t *conn_state = malloc(sizeof(http_conn_state_t));

    *conn_state = (http_conn_state_t){
        .request = (http_raw_request_t){
            .parser = {},
            .url = chunked_str_new(),
            .packets = NULL,
            .headers = header_map_new(),
        },
        .response = (http_raw_response_t){
            .response_text = http_response,
            .sent = 0,
        },
    };

    // TODO: implement body
    static const llhttp_settings_t parser_settings = {
        // .on_body = __test_3,
        .on_chunk_complete = on_chunk_header,
        .on_chunk_header = on_chunk_header,
        .on_header_field = on_header_field,
        .on_header_field_complete = on_header_field_complete,
        .on_header_value = on_header_value,
        .on_header_value_complete = on_header_value_complete,
        .on_headers_complete = on_headers_complete,
        // .on_message_begin = __test_1,
        // .on_message_complete = on_message_complete,
        // .on_status = __test_3,
        // .on_status_complete = __test_1,
        .on_url = on_url,
        .on_url_complete = on_url_complete,
    };

    llhttp_init(&conn_state->request.parser, HTTP_REQUEST, &parser_settings);

    tcp_arg(client_pcb, conn_state);
    tcp_sent(client_pcb, (tcp_sent_fn)server_sent);
    tcp_recv(client_pcb, (tcp_recv_fn)server_recv);

#define POLL_TIME_S 5
    tcp_poll(client_pcb, (tcp_poll_fn)server_poll, POLL_TIME_S * 2);
    tcp_err(client_pcb, (tcp_err_fn)server_err);

    return ERR_OK;
}
