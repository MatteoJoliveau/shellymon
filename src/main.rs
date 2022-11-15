use std::path::PathBuf;

use clap::Parser;
use shellymon::{config::Config, telemetry, Application};

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

async fn run(cfg: Config) -> anyhow::Result<()> {
    let app = Application::new(cfg);
    tracing::info!("ShellyMon started");
    app.start().await
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Args::parse();
    let cfg = Config::new(args.config).expect("config loading failed");
    telemetry::init(&cfg);

    tracing::info!(?cfg, "configuration loaded");

    if let Err(err) = run(cfg).await {
        tracing::error!("{err:?}");
        std::process::exit(1);
    }
}
