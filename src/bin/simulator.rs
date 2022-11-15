use std::path::PathBuf;

use clap::Parser;
use shellymon::{config::Config};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 'C',
        long,
        env = "SHELLYMON_CONFIG_PATH",
        default_value = "config.yml"
    )]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Args::parse();
    let cfg = Config::new(args.config).expect("config loading failed");
    tracing_subscriber::fmt().compact().init();

    tracing::info!(?cfg, "configuration loaded");
}