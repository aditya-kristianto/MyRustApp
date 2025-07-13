use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
    pub success: bool,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>
}

impl JsonResponse {
    pub fn new(success: bool, code: u16, message: Option<String>, data: Option<String>) -> Self {
        JsonResponse {
            success,
            code,
            message,
            data
        }
    }
}