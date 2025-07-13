use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use super::enum::DataValue;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"status": 200, "message": "OK"}))]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default = "default_status")]
    pub status: u16,
}

impl Response {
    pub fn new(data: Option<HashMap<String, DataValue>>, error: Option<Vec<Error>>, message: Option<String>, meta: Option<Meta>, status: u16) -> Self {
        Response {
            data: data,
            error: error,
            message: message,
            meta: meta,
            status: status,
        }
    }
}