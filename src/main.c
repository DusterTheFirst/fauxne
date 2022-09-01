#include "hardware/clocks.h"
#include "hardware/interp.h"
#include "hardware/timer.h"
#include "hardware/watchdog.h"
#include "pico/cyw43_arch.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "lwip/pbuf.h"
#include "lwip/tcp.h"

#include "log.h"

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

    // Interpolator example code
    interp_config cfg = interp_default_config();
    // Now use the various interpolator library functions for your use case
    // e.g. interp_config_clamp(&cfg, true);
    //      interp_config_shift(&cfg, 2);
    // Then set the config
    interp_set_config(interp0, 0, &cfg);

    // Timer example code - This example fires off the callback after 2000ms
    add_alarm_in_ms(2000, alarm_callback, NULL, false);

    INFO("Hello, world!");

    return 0;
}
