use futures::future::try_join_all;
use anyhow::Context as _;
use self::config::Config;

pub mod config;
pub mod telemetry;

mod receiver;
mod exporter;

pub struct Application {
    cfg: Config,
}

impl Application {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let receivers = self.cfg.receivers.into_iter().map(|recv| tokio::spawn(receiver::start(recv)));
        try_join_all(receivers).await.context("receivers stopped")?;

        Ok(())
    }
}