use std::str::FromStr;

use serde::Deserialize;

use tracing_subscriber::{
    fmt,
    prelude::*,
    EnvFilter, Registry,
};

use crate::config::Config;

pub fn init(cfg: &Config) {
    let level = cfg.log().level();
    let format = cfg.log().format();

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_from(level))
        .expect("Env Filter");

    let json = format.is_json().then(|| {
        fmt::layer()
            .json()
            .with_current_span(true)
            .flatten_event(true)
    });

    let text = format.is_text().then(|| fmt::layer().compact());

    let pretty = format.is_pretty().then(|| fmt::layer().pretty());

    Registry::default()
        .with(env_filter)
        .with(json)
        .with(text)
        .with(pretty)
        .init()
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Text,
    Pretty,
}

impl LogFormat {
    fn is_json(&self) -> bool {
        matches!(self, Self::Json)
    }

    fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    fn is_pretty(&self) -> bool {
        matches!(self, Self::Pretty)
    }
}
