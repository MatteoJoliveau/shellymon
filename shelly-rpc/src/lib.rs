pub mod components;
pub mod notification;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestFrame<T> {
    pub jsonrpc: Option<String>,
    pub id: String,
    pub src: String,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseFrame<T, E> {
    pub id: String,
    pub src: String,
    pub dst: String,
    result: Option<T>,
    error: Option<E>,
}

impl<T, E> ResponseFrame<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        self.result.ok_or_else(|| {
            self.error
                .expect("JSON RPC response must contain either a result or an error!")
        })
    }
}
