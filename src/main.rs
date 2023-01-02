#![warn(clippy::all, clippy::pedantic)]
use log::LevelFilter;
use simplelog::{ColorChoice, Config as SimpleLogConfig, TermLogger, TerminalMode};
use std::fs::File;
use teevee::{stream_videos, Config};

fn main() {
    let config_reader = File::open("config.yml").expect("failed to open config file");
    let config =
        serde_yaml::from_reader::<_, Config>(config_reader).expect("failed to parse config");

    setup_logger(config.log_level());
    stream_videos(&config).expect("stream setup failed");
}

fn setup_logger(level: LevelFilter) {
    TermLogger::new(
        level,
        SimpleLogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
}
