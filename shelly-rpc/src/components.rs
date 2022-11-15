use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ble {}

#[derive(Debug, Deserialize)]
pub struct Cloud {
    pub connected: bool,
}

#[derive(Debug, Deserialize)]
pub struct Mqtt {
    pub connected: bool,
}

#[derive(Debug, Deserialize)]
pub struct Switch {
    pub id: usize,
    pub output: bool,
    pub source: String,
}

