#[macro_use]
extern crate log;

use async_std::task;

use dotenv::dotenv;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger, TermLogger, TerminalMode};
use std::env;
use voice::client::VoiceClient;

mod voice;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let log_level = if cfg!(debug_assertions) {
        // LevelFilter::Trace
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let mut config = ConfigBuilder::new();
    config.add_filter_ignore_str("tungstenite");
    config.add_filter_ignore_str("async_tungstenite");
    config.add_filter_ignore_str("mio");
    let config = config.build();

    if let Err(_) = TermLogger::init(log_level, config.clone(), TerminalMode::Mixed) {
        SimpleLogger::init(log_level, config).expect("No logger should be already set");
    };

    let client = VoiceClient::connect().await.unwrap();
    info!("Connected to discord gateway");

    let handler_client = client.clone();
    ctrlc::set_handler(move || {
        let handler_client = handler_client.clone();

        warn!("Closing connection");

        task::spawn(async move {
            handler_client.disconnect().await.unwrap();
        });

        // process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    client
        .login(&env::var("TOKEN").expect("TOKEN env var missing"))
        .await
        .unwrap();
}
