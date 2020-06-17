// TODO: #![warn(missing_docs)]
//! Utilities for the ye_olde_discord binary ecosystem

#[macro_use]
extern crate log;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive_flexi_logger_view::FlexiLoggerView;
use flexi_logger::{LogTarget, Logger};
use std::thread;

pub mod voice;

pub fn start_ui() {
    thread::spawn(|| {
        // Creates the cursive root - required for every application.
        let mut siv = Cursive::default();

        siv.show_debug_console();

        Logger::with_env_or_str(if cfg!(debug_assertions) {
            // "trace"
            "ye-olde-discord=debug"
        } else {
            "ye-olde-discord=info"
        })
        .log_target(LogTarget::Writer(
            cursive_flexi_logger_view::cursive_flexi_logger(&siv),
        ))
        .suppress_timestamp()
        .format(flexi_logger::colored_with_thread)
        .start()
        .expect("failed to initialize logger!");

        siv.add_layer(FlexiLoggerView); // omit `scrollable` to remove scrollbars

        // Creates a dialog with a single "Quit" button
        // siv.add_layer(
        //     Dialog::around(TextView::new("Hello Dialog!"))
        //         .title("Cursive")
        //         .button("Quit", |s| s.quit()),
        // );

        // Starts the event loop.
        siv.run();
    });
}
