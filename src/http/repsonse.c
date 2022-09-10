#include "error.h"
#include "http/response.h"

err_t response_send(http_raw_response_t *response,
                    struct tcp_pcb *client_pcb) {
    size_t remaining = response_remaining_len(response);

    TRACE("sending %d remaining bytes", remaining);

    for (size_t i = 0; i < response->data.length; i++) {
        str_t *str = &response->data.buffer[i];

        DBG("%d", tcp_sndbuf(client_pcb));

        size_t len = MIN(str->len, tcp_sndbuf(client_pcb));

        TRACE("sending %d bytes of a %d byte chunk", len, str->len);

        // FIXME: only send tcp_sndbuf(client_pcb) max
        TCP_TRY(
            tcp_write(client_pcb, str->ptr, len, 0),
            "encountered error writing to tcp connection");
    }

    TCP_TRY(tcp_output(client_pcb),
            "Encountered error outputting to tcp connection");

    return ERR_OK;
}
