#[macro_use]
extern crate log;

use async_std::task;

use dotenv::dotenv;
use log::LevelFilter;
use std::{env, io::stdout, process};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};
use ye_olde_discord::ui::logger::{TargetFilter, TuiLogger, TuiLoggerConfig};
use ye_olde_discord::{ui::UI, voice::client::VoiceClient};

#[async_std::main]
async fn main() {
    dotenv().ok();

    let log_level = if cfg!(debug_assertions) {
        // LevelFilter::Trace
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TuiLogger::init(
        TuiLoggerConfig::default()
            // .set_level(log_level)
            // .set_filter(TargetFilter::Whitelist(vec!["ye_olde_discord".into()])),
    )
    .expect("Failed to init logger");

    // Init tui for UI drawing
    let ui = UI::new(1000);

    ui.clone().run();

    trace!("1");
    debug!("2");
    info!("3");
    warn!("4");
    error!("5");

    // let mut config = ConfigBuilder::new();
    // config.add_filter_ignore_str("tungstenite");
    // config.add_filter_ignore_str();
    // config.add_filter_ignore_str();
    // // TODO: config.set_thread_mode(ThreadMode::Both);
    // let config = config.build();

    // if let Err(_) = TermLogger::init(log_level, config.clone(), TerminalMode::Mixed) {
    //     SimpleLogger::init(log_level, config).expect("No logger should be already set");
    // };

    let client = VoiceClient::connect().await.unwrap();
    info!("Connected to discord gateway");

    let handler_client = client.clone();
    let handler_ui = ui.clone();
    ctrlc::set_handler(move || {
        let handler_client = handler_client.clone();

        warn!("Closing connection");

        task::block_on(async move {
            handler_client.disconnect().await.ok();
        });

        handler_ui.clone().stop().ok();

        process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    client
        .login(&env::var("TOKEN").expect("TOKEN env var missing"))
        .await
        .unwrap();
}
