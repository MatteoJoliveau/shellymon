use std::{collections::HashMap, fmt::Display, ops::Deref};

use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct NotificationFrame {
    pub src: String,
    pub dst: String,
    #[serde(flatten)]
    pub payload: Notification,
}

impl<'a> TryFrom<&'a [u8]> for NotificationFrame {
    type Error = serde_json::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(bytes)
    }
}

impl AsRef<Notification> for NotificationFrame {
    fn as_ref(&self) -> &Notification {
        &self.payload
    }
}

impl Deref for NotificationFrame {
    type Target = Notification;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum Notification {
    NotifyStatus(NotifyStatus),
    NotifyFullStatus(NotifyStatus),
    NotifyEvent(NotifyEvent),
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.method().fmt(f)
    }
}

impl Notification {
    pub fn method(&self) -> &'static str {
        match self {
            Notification::NotifyStatus(_) => "NotifyStatus",
            Notification::NotifyFullStatus(_) => "NotifyFullStatus",
            Notification::NotifyEvent(_) => "NotifyEvent",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NotifyStatus {
    #[serde(rename = "ts")]
    pub timestamp: f64,
    #[serde(flatten)]
    components: HashMap<String, serde_json::Value>,
}

impl NotifyStatus {
    pub fn component<T: DeserializeOwned>(
        &self,
        name: &str,
    ) -> Result<Option<T>, serde_json::Error> {
        self.components
            .get(name)
            .map(|component| serde_json::from_value(component.clone()))
            .transpose()
    }
}

#[derive(Debug, Deserialize)]
pub struct NotifyEvent {
    #[serde(rename = "ts")]
    pub timestamp: f64,
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "ts")]
    pub timestamp: f64,
    pub id: i64,
    pub event: String,
    #[serde(flatten)]
    payload: serde_json::Value,
}

impl Event {
    pub fn payload<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::components::*;

    use super::*;

    #[test]
    fn it_deserializes_a_status() {
        let raw = json!({
           "src": "shellypro4pm-f008d1d8b8b8",
           "dst": "user_1",
           "method": "NotifyStatus",
           "params": {
              "ts": 1631186545.04,
              "switch:0": {
                 "id": 0,
                 "output": true,
                 "source": "button"
              }
           }
        });

        let status = serde_json::from_value::<NotificationFrame>(raw).unwrap();
        assert_eq!(&status.src, "shellypro4pm-f008d1d8b8b8");
        assert_eq!(&status.dst, "user_1");
        assert_eq!(status.method(), "NotifyStatus");
        assert!(matches!(status.payload, Notification::NotifyStatus(_)));
        let Notification::NotifyStatus(status) = status.payload else {
            panic!("payload is not NotifyStatus");
        };
        assert_eq!(status.timestamp, 1631186545.04);
        let switch = status.component::<Switch>("switch:0").unwrap().unwrap();
        assert_eq!(switch.id, 0);
        assert_eq!(switch.output, true);
        assert_eq!(&switch.source, "button");
    }

    #[test]
    fn it_deserializes_a_full_status() {
        let raw = json!({
           "src": "shellyplusht-f008d1d8b8b8",
           "dst": "user_1",
           "method": "NotifyFullStatus",
           "params": {
              "ts": 1631186545.04,
              "ble": {},
              "cloud": {
                 "connected": false
              },
              "mqtt": {
                 "connected": false
              },
              "sys": {
                 "mac": "F008D1E62338",
                 "restart_required": false,
                 "time": null,
                 "unixtime": null,
                 "uptime": 41,
                 "ram_size": 254948,
                 "ram_free": 146620,
                 "fs_size": 458752,
                 "fs_free": 229376,
                 "cfg_rev": 0,
                 "available_updates": {}
              },
              "wifi": {
                 "sta_ip": null,
                 "status": "disconnected",
                 "ssid": null,
                 "rssi": 0
              },
              "temperature:0": {
                 "id": 0,
                 "tC": 20.0,
                 "tF": 68.0
              },
              "humidity:0": {
                 "id": 0,
                 "rh": 50.0
              },
              "devicepower:0": {
                 "id": 0,
                 "battery": {
                    "V": 4.59,
                    "percent": 11
                 },
                 "external": {
                    "present": false
                 }
              },
              "ht_ui": {}
           }
        });

        let status = serde_json::from_value::<NotificationFrame>(raw).unwrap();
        assert_eq!(&status.src, "shellyplusht-f008d1d8b8b8");
        assert_eq!(&status.dst, "user_1");
        assert_eq!(status.method(), "NotifyFullStatus");
        assert!(matches!(status.payload, Notification::NotifyFullStatus(_)));
        let Notification::NotifyFullStatus(status) = status.payload else {
            panic!("payload is not NotifyFullStatus");
        };
        assert_eq!(status.timestamp, 1631186545.04);

        status.component::<Ble>("ble").unwrap().unwrap();

        let Cloud { connected } = status.component::<Cloud>("cloud").unwrap().unwrap();
        assert!(!connected);

        let Mqtt { connected } = status.component::<Mqtt>("mqtt").unwrap().unwrap();
        assert!(!connected);
    }

    #[test]
    fn it_deserializes_an_event() {
        let raw = json!({
           "src": "shellypro4pm-f008d1d8b8b8",
           "dst": "user_1",
           "method": "NotifyEvent",
           "params": {
              "ts": 1631266595.44,
              "events": [
                 {
                    "component": "input:0",
                    "id": 0,
                    "event": "single_push",
                    "ts": 1631266595.44
                 }
              ]
           }
        });

        let event = serde_json::from_value::<NotificationFrame>(raw).unwrap();
        assert_eq!(&event.src, "shellypro4pm-f008d1d8b8b8");
        assert_eq!(&event.dst, "user_1");
        assert_eq!(event.method(), "NotifyEvent");
        assert!(matches!(event.payload, Notification::NotifyEvent(_)));
        let Notification::NotifyEvent(event) = event.payload else {
            panic!("payload is not NotifyEvent");
        };
        assert_eq!(event.timestamp, 1631266595.44);
        
        let event = event.events.first().unwrap();
        assert_eq!(event.timestamp, 1631266595.44);
        assert_eq!(event.id, 0);
        assert_eq!(&event.event, "single_push");
        
        #[derive(Deserialize)]
        struct Component {
            component: String,
        }
        let Component { component } = event.payload().unwrap();
        assert_eq!(&component, "input:0");
    }
}
