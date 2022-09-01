#include "hardware/clocks.h"
#include "hardware/interp.h"
#include "hardware/timer.h"
#include "hardware/watchdog.h"
#include "pico/stdlib.h"
#include <stdio.h>

#include "log.h"

int64_t alarm_callback(alarm_id_t id, void *user_data) {
    // Suppress -Wunused-parameter
    (void)id;
    (void)user_data;

    // Put your timeout handler code in here
    return 0;
}

int main(void) {
    stdio_init_all();

    if (watchdog_caused_reboot()) {
        WARN("Reboot caused by watchdog");
    } else {
        TRACE("Normal reboot");
    }

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
