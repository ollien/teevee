#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config as SimpleLogConfig, TermLogger, TerminalMode};
use std::fs::File;
use teevee::{stream_videos, Config};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, default_value = "config.yml")]
    config_file: String,
}

fn main() {
    let args = Args::parse();

    let config_reader = File::open(args.config_file).expect("failed to open config file");
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
