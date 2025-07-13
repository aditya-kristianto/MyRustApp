use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use std::collections::HashMap;

// #[path = "../bin/duuid/models/duuid_models.rs"]
// mod models;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, IntoParams, Serialize, ToSchema)]
#[into_params(parameter_in = Header)]
// #[serde(rename_all = "PascalCase")]
pub struct Header {
    /// Accept Header
    #[param(example = "application/json")]
    #[serde(rename="accept")]
    pub accept: String,
    /// Authorization Header 
    // #[validate(length(min = 1, max = 20))]
    // #[param(example = "Bearer 12345")]
    // pub authorization: String,
    /// Content Type Header
    #[param(example = "application/json")]
    #[serde(rename="Content-Type")]
    pub content_type: String,
    /// Test Header
    #[param(example = "hello-world")]
    #[serde(rename="test")]
    pub test: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub page: u8,
    pub limit: u8,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"status": "OK", "message": "OK", "data": {"uuid": ""}}))]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub data: Option<models::duuid_model::DUUID>,
    pub data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub status: String,
}