#include "http.h"
#include "chunked_str.h"
#include "error.h"
#include "http/callbacks.h"
#include "http/request.h"
#include "http/response.h"
#include "log.h"

err_t http_server_init(ip4_addr_t *ip,
                       uint16_t port,
                       http_server_t *server) {
    if (!server) {
        ERROR("provided null pointer to http server");

        return ERR_VAL;
    } else {
        TRACE("allocated http server");
    }

    struct tcp_pcb *pcb = tcp_new_ip_type(IPADDR_TYPE_V4);

    if (pcb == NULL) {
        ERROR("failed to create tcp protocol control block");

        return ERR_MEM;
    } else {
        TRACE("created tcp_pcb");
    }

    TCP_TRY(tcp_bind(pcb, ip, port),
            "failed to bind to %d.%d.%d.%d:%d",
            ip4_addr1(ip), ip4_addr2(ip), ip4_addr3(ip), ip4_addr4(ip), port);

    TRACE("bound tcp_pcb to ip");

    err_t listen_err;
    server->server_pcb =
        tcp_listen_with_backlog_and_err(pcb, 1, &listen_err);
    if (server->server_pcb == NULL) {
        TCP_ERROR(listen_err, "failed to setup tcp pcb for listening");

        tcp_close(pcb);
        free(server);

        return listen_err;
    }

    TRACE("tcp server listening");

    tcp_arg(server->server_pcb, server);
    tcp_accept(server->server_pcb, (tcp_accept_fn)callback_accept);

    TRACE("tcp server setup");

    INFO("HTTP server listening at http://%d.%d.%d.%d:%d",
         ip4_addr1(http_server_local_ip(server)),
         ip4_addr2(http_server_local_ip(server)),
         ip4_addr3(http_server_local_ip(server)),
         ip4_addr4(http_server_local_ip(server)),
         http_server_local_port(server));

    return ERR_OK;
}

err_t http_server_close(http_server_t *server) {
    err_t tcp_close_err = tcp_close(server->server_pcb);

    free(server);

    return tcp_close_err;
}

chunked_str_t http_server_respond(http_raw_request_t *request) {
#include "static_files.h"

    (void)static_file_js__event_js;
    (void)static_file_js__index_js;

    // static const str_t event_stream = str(
    //     "HTTP/1.1 200 OK\r\n"
    //     "Content-Type: text/event-stream\r\n"
    //     "\r\n"
    //     ": welcome\n");

#define HTTP_OK "HTTP/1.1 200 OK\r\n"
#define HTTP_NOT_FOUND "HTTP/1.1 404 Not Found\r\n"

    // TODO: save on requests, bundle html with trunkrs.dev
#define CONTENT_TYPE_HTML "Content-Type: text/html\r\n"
#define CONTENT_TYPE_CSS "Content-Type: text/css\r\n"
#define CONTENT_TYPE_JS "Content-Type: text/javascript\r\n"
#define CONTENT_TYPE_EVENT_STREAM "Content-Type: text/event-stream\r\n"

#define HTTP_BODY_SEPARATOR "\r\n"

    if (chunked_str_eq(&request->url, str("/"))) {
        DEBUG("INDEX");

        chunked_str_t http_response = chunked_str_new_with_capacity(2);
        chunked_str_push(&http_response,
                         str(HTTP_OK CONTENT_TYPE_HTML
                                 HTTP_BODY_SEPARATOR));
        chunked_str_push(&http_response, static_file_index_html);

        return http_response;
    } else {
        DEBUG("404");

        chunked_str_t http_response = chunked_str_new_with_capacity(2);
        chunked_str_push(&http_response,
                         str(HTTP_NOT_FOUND CONTENT_TYPE_HTML
                                 HTTP_BODY_SEPARATOR));
        chunked_str_push(&http_response, static_file_404_html);

        return http_response;
    }
}
