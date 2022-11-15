use self::config::Config;
use anyhow::Context as _;
use futures::future::try_join_all;

pub mod config;
pub mod telemetry;

mod exporter;
mod receiver;

pub struct Application {
    cfg: Config,
}

impl Application {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let receivers = self
            .cfg
            .receivers
            .into_iter()
            .map(|recv| tokio::spawn(receiver::start(recv)));
        try_join_all(receivers).await.context("receivers stopped")?;

        Ok(())
    }
}
