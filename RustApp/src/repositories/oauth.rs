use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OauthAccessTokens {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub name: String,
    pub scopes: String,
    pub revoked: bool,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub expires_at: Option<SystemTime>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OauthAuthCodes {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub scopes: String,
    pub revoked: bool,
    pub expires_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OauthClients {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub secret: String,
    pub provider: String,
    pub redirect: String,
    pub personal_access_client: bool,
    pub password_client: bool,
    pub revoked: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OauthPersonalAccessClients {
    pub id: String,
    pub client_id: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OauthRefreshTokens {
    pub id: String,
    pub access_token_id: String,
    pub revoked: bool,
    pub expires_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResets {
    pub email: String,
    pub token: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalAccessTokens {
    pub id: String,
    pub tokenable_type: String,
    pub tokenable_id: String,
    pub name: String,
    pub token: String,
    pub abilities: String,
    pub last_used_at: String,
    pub expires_at: SystemTime,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"status": 200, "message": "OK"}))]
pub struct Response {
    #[serde(default = "default_status")]
    pub status: u8,
    #[serde(default = "default_message")]
    pub message: String,
    pub meta: Option<Meta>
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Meta {
    pub page: u8,
    pub limit: u8
}

fn default_status() -> u8 {
    200
}

fn default_message() -> String {
    "/".to_string()
}