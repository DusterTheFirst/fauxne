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

#include "log.h"

bi_decl(bi_program_name("fauxne"));
bi_decl(bi_program_description("POTS FXO emulator"));
bi_decl(bi_program_version_string("0.0"));
bi_decl(bi_program_url("https://github.com/DusterTheFirst/fauxne"));

int64_t alarm_callback(alarm_id_t id, void *user_data) {
    // Suppress -Wunused-parameter
    (void)id;
    (void)user_data;

    // Put your timeout handler code in here
    return 0;
}

static const char *const ssid = "fauxne";
static const char *const psk = "fake phone";

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

    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 1);

    ip4_addr_t gateway, netmask;
    IP4_ADDR(&gateway, 192, 168, 69, 1);
    IP4_ADDR(&netmask, 255, 255, 255, 0);

    // Start the dhcp server
    dhcp_server_t dhcp_server;
    dhcp_server_init(&dhcp_server, &gateway, &netmask);

    // // Interpolator example code
    // interp_config cfg = interp_default_config();
    // // Now use the various interpolator library functions for your use case
    // // e.g. interp_config_clamp(&cfg, true);
    // //      interp_config_shift(&cfg, 2);
    // // Then set the config
    // interp_set_config(interp0, 0, &cfg);

    // // Timer example code - This example fires off the callback after 2000ms
    // add_alarm_in_ms(2000, alarm_callback, NULL, false);

    const struct tcp_pcb *protocol_control_block =
        tcp_new_ip_type(IPADDR_TYPE_V4);

    if (protocol_control_block == NULL) {
        ERROR("failed to create tcp protocol control block");

        return 1;
    }

    while (true) {
        sleep_until(at_the_end_of_time);
    }

    return 0;
}
