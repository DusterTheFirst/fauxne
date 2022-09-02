#include "hardware/clocks.h"
#include "hardware/interp.h"
#include "hardware/timer.h"
#include "hardware/watchdog.h"
#include "pico/binary_info.h"
#include "pico/cyw43_arch.h"
#include "pico/stdlib.h"
#include "tusb.h"
#include <stdio.h>

#include "lwip/pbuf.h"
#include "lwip/tcp.h"

#include "dhcpserver.h"

#include "error.h"
#include "log.h"

bi_decl(bi_program_name("fauxne"));
bi_decl(bi_program_description("POTS FXO emulator"));
bi_decl(bi_program_version_string("0.0"));
bi_decl(bi_program_url("https://github.com/DusterTheFirst/fauxne"));

#define TCP_PORT 80
#define POLL_TIME_S 5

int64_t alarm_callback(alarm_id_t id, void *user_data) {
    // Suppress -Wunused-parameter
    (void)id;
    (void)user_data;

    // Put your timeout handler code in here
    return 0;
}

static const char *const ssid = "fauxne";
static const char *const psk = "fake phone";

static const char http_response[] =
    "HTTP/1.1 200 OK\r\n"
    "Content-Length: 13\r\n"
    "Content-Type: text/html\r\n"
    "\r\n"
    "<h1>piss</h1>";

static const size_t http_response_length = sizeof(http_response) / sizeof(char);

typedef struct _http_server {
    size_t sent_bytes;
} http_server;

static http_server *http_server_init(void) {
    http_server *state = malloc(sizeof(http_server));

    state->sent_bytes = 0;

    if (!state) {
        ERROR("failed to allocate state");

        return NULL;
    }

    return state;
}

static err_t send_http_response(http_server *server,
                                struct tcp_pcb *client_pcb) {
    TRY(tcp_write(client_pcb, http_response + server->sent_bytes,
                  http_response_length - server->sent_bytes, 0),
        "Encountered error writing to tcp connection");

    TRY(tcp_output(client_pcb),
        "Encountered error outputting to tcp connection");

    return ERR_OK;
}

static err_t tcp_server_sent(void *arg, struct tcp_pcb *client_pcb,
                             u16_t len) {
    http_server *server = (http_server *)arg;

    DEBUG("TCP server sent %u bytes\n", len);

    server->sent_bytes += len;

    if (server->sent_bytes == http_response_length) {
        TRACE("DONE");
        TRY(tcp_close(client_pcb), "Encountered error trying to close pcb");
        server->sent_bytes = 0;
    }

    return ERR_OK;
}

static err_t tcp_server_recv(void *arg, struct tcp_pcb *client_pcb,
                             struct pbuf *p, err_t err) {
    __unused http_server *server = (http_server *)arg;

    TRY(err, "Encountered error receiving data");

    DEBUG("Received %d bytes", p->len);
    tcp_recved(client_pcb, p->len);
    pbuf_free(p);

    return ERR_OK;
}

static err_t tcp_server_poll(void *arg, struct tcp_pcb *client_pcb) {
    __unused http_server *server = (http_server *)arg;

    TRY(send_http_response(server, client_pcb), "Failed to send http response");

    return ERR_OK;
}

static void tcp_server_err(void *arg, err_t err) {
    __unused http_server *server = (http_server *)arg;

    ERROR("Encountered a TCP server error %d", err);
}

static err_t tcp_server_accept(void *arg, struct tcp_pcb *client_pcb,
                               err_t err) {
    http_server *server = (http_server *)arg;

    if (err != ERR_OK || client_pcb == NULL) {
        ERROR("Failure in accept");

        return ERR_VAL;
    }
    INFO("Client connected");

    tcp_arg(client_pcb, server);
    tcp_sent(client_pcb, tcp_server_sent);
    tcp_recv(client_pcb, tcp_server_recv);
    tcp_poll(client_pcb, tcp_server_poll, POLL_TIME_S * 2);
    tcp_err(client_pcb, tcp_server_err);

    TRY(send_http_response(server, client_pcb), "Failed to send http response");

    // err_t close_err = tcp_close(client_pcb);

    // if (close_err != ERR_OK) {
    //     ERROR("Encountered error closing tcp connection");

    //     return close_err;
    // }

    return ERR_OK;
}

int main(void) {
    stdio_init_all();

    // Wait for usb connection if it is the only configured STDIO output
#if LIB_PICO_STDIO_USB && !LIB_PICO_STDIO_UART && !LIB_PICO_STDIO_SEMIHOSTING
    while (!tud_cdc_connected()) {
        sleep_ms(100);
    }
    TRACE("USB CDC Connected, starting boot process");
#endif

    TRACE("TRACE");
    DEBUG("DEBUG");
    INFO("INFO");
    WARN("WARN");
    ERROR("ERROR");

    if (watchdog_enable_caused_reboot()) {
        WARN("Reboot caused by watchdog timeout");
    } else if (watchdog_caused_reboot()) {
        WARN("Reboot caused by watchdog manually");
    } else {
        TRACE("Normal reboot");
    }

    if (cyw43_arch_init_with_country(CYW43_COUNTRY_NETHERLANDS)) {
        ERROR("failed to initialise cyw43 for NL");
        return 1;
    }
    DEBUG("initialized cyw43 for NL");

    cyw43_arch_enable_ap_mode(ssid, psk, CYW43_AUTH_WPA2_AES_PSK);
    INFO("access point mode enabled");
    DBG_str(ssid);
    DBG_str(psk);

    ip4_addr_t gateway, netmask;
    IP4_ADDR(&gateway, 192, 168, 4, 1);
    IP4_ADDR(&netmask, 255, 255, 255, 0);

    // Start the dhcp server
    dhcp_server_t dhcp_server;
    dhcp_server_init(&dhcp_server, &gateway, &netmask);

    INFO("DHCP server started");

    // // Interpolator example code
    // interp_config cfg = interp_default_config();
    // // Now use the various interpolator library functions for your use case
    // // e.g. interp_config_clamp(&cfg, true);
    // //      interp_config_shift(&cfg, 2);
    // // Then set the config
    // interp_set_config(interp0, 0, &cfg);

    // // Timer example code - This example fires off the callback after 2000ms
    // add_alarm_in_ms(2000, alarm_callback, NULL, false);

    struct tcp_pcb *pcb = tcp_new_ip_type(IPADDR_TYPE_V4);

    if (pcb == NULL) {
        ERROR("failed to create tcp protocol control block");

        return 1;
    }

    {
        const err_t bind_err = tcp_bind(pcb, &gateway, TCP_PORT);
        if (bind_err != ERR_OK) {
            ERROR("failed to bind to %d.%d.%d.%d:%d (error %d)",
                  ip4_addr1(&gateway), ip4_addr2(&gateway),
                  ip4_addr3(&gateway), ip4_addr4(&gateway),
                  TCP_PORT, bind_err);

            return bind_err;
        }
    }

    struct tcp_pcb *server_pcb;
    {
        err_t listen_err;
        server_pcb = tcp_listen_with_backlog_and_err(pcb, 1, &listen_err);
        if (server_pcb == NULL) {
            ERROR("failed to listen (error %d)", listen_err);

            if (pcb != NULL) {
                tcp_close(pcb);
            }

            return false;
        }
    }

    http_server *server = http_server_init();
    if (server == NULL) {
        return 7;
    }

    tcp_arg(server_pcb, server);
    tcp_accept(server_pcb, tcp_server_accept);

    INFO("TCP server listening at http://%d.%d.%d.%d:%d ",
         ip4_addr1(&server_pcb->local_ip), ip4_addr2(&server_pcb->local_ip),
         ip4_addr3(&server_pcb->local_ip), ip4_addr4(&server_pcb->local_ip),
         server_pcb->local_port);

    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 1);

    while (true) {
        sleep_until(at_the_end_of_time);
    }

    tcp_close(server_pcb);

    dhcp_server_deinit(&dhcp_server);
    cyw43_arch_deinit();

    return 0;
}
