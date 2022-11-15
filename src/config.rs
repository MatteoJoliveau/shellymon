use std::path::PathBuf;

use config::{ConfigError, Environment, File};
use serde::Deserialize;

use crate::{telemetry::LogFormat, receiver::ReceiverConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    log: Log,
    pub receivers: Vec<ReceiverConfig>,
}

impl Config {
    pub fn new(path: PathBuf) -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .set_default("log.level", "shellymon=info")?
            .set_default("log.format", "json")?
            .add_source(File::from(path))
            .add_source(
                Environment::with_prefix("SHELLYMON")
                    .separator("_")
                    .list_separator(","),
            )
            .build()?;

        cfg.try_deserialize()
    }

    pub fn log(&self) -> &Log {
        &self.log
    }
}

#[derive(Debug, Deserialize)]
pub struct Log {
    level: String,
    format: LogFormat,
}

impl Log {
    pub fn level(&self) -> &str {
        &self.level
    }

    pub fn format(&self) -> &LogFormat {
        &self.format
    }
}
