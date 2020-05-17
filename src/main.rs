#[macro_use]
extern crate log;

use dotenv::dotenv;
use simplelog::{LevelFilter, SimpleLogger, TermLogger, TerminalMode, ConfigBuilder};
use std::env;
use voice::client::VoiceClient;

mod voice;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let log_level = if cfg!(debug_assertions) {
        /* LevelFilter::Trace */ LevelFilter::Debug
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

    let mut client = VoiceClient::connect().await.unwrap();
    info!("Connected to discord gateway");

    client
        .login(&env::var("TOKEN").expect("TOKEN env var missing"))
        .await
        .unwrap();
}
