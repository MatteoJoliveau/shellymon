use std::time::Duration;

use paho_mqtt::{ConnectOptionsBuilder, CreateOptionsBuilder};
use secstr::SecUtf8;
use serde::Deserialize;

use super::ReceiverError;

#[derive(Debug, Deserialize)]
pub struct MqttOptions {
    host: String,
    port: u16,
    username: String,
    password: SecUtf8,
    client_id: Option<String>,
}

pub struct MqttReceiver {
    name: String,
    opts: MqttOptions,
}

impl MqttReceiver {
    pub fn new(name: String, opts: MqttOptions) -> Self {
        Self { name, opts }
    }

    fn server_uri(&self) -> String {
        format!("tcp://{}:{}", self.opts.host, self.opts.port)
    }

    pub async fn start(self) -> Result<(), ReceiverError> {
        let server_uri = self.server_uri();
        let mut client = CreateOptionsBuilder::new()
            .client_id(self.opts.client_id.unwrap_or_default())
            .server_uri(server_uri)
            .create_client()?;

        let opts = ConnectOptionsBuilder::new()
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
            .user_name(self.opts.username)
            .password(self.opts.password.unsecure())
            .finalize();

        let mut stream = client.get_stream(32);

        client.connect_with_callbacks(
            opts,
            |_client, port| {
                tracing::info!("MQTT receiver started {port}");
            },
            |_client, port, errno| {
                tracing::error!(
                    "MQTT receiver failed to start {port} {}",
                    paho_mqtt::error_message(errno)
                );
            },
        ).await?;

        client.subscribe_many(&["hello"], &[paho_mqtt::QOS_1]).await?;

        while let Some(msg) = stream.recv().await? {
            tracing::info!(%msg, "received");
        }

        Ok(())
    }
}
