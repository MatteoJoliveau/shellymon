use serde::Deserialize;

mod error;
pub use error::ReceiverError;

use self::mqtt::{MqttOptions, MqttReceiver};
pub mod mqtt;

#[derive(Debug, Deserialize)]
pub struct ReceiverConfig {
    name: Option<String>,
    #[serde(flatten)]
    options: ReceiverOptions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverOptions {
    Mqtt(MqttOptions),
}

impl ReceiverOptions {
    fn label(&self) -> &'static str {
        match self {
            ReceiverOptions::Mqtt(_) => "mqtt",
        }
    }
}

pub async fn start(cfg: ReceiverConfig) -> Result<(), ReceiverError> {
    let name = cfg
        .name
        .unwrap_or_else(|| format!("{} receiver", cfg.options.label()));
    match cfg.options {
        ReceiverOptions::Mqtt(mqtt) => MqttReceiver::new(name, mqtt).start().await,
    }
}
