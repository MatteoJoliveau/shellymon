use async_channel::RecvError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReceiverError {
    #[error("mqtt client error: {0}")]
    Mqtt(#[from] paho_mqtt::Error),
    #[error("mqtt receive error: {0}")]
    Receive(#[from] RecvError),
}
