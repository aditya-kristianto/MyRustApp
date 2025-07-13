use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccessToken {
    pub client_id: Uuid,
    pub user_id: Uuid,
    pub token_value: String,
    pub scope: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

pub struct Client {
    pub _client_id: Uuid,
    pub _client_secret: String,
    pub _client_name: String,
    pub _redirect_uri: String,
    pub _created_at: DateTime<Utc>,
    pub _created_by: Uuid,
    pub _updated_at: Option<DateTime<Utc>>,
    pub _updated_by: Option<Uuid>,
    pub _deleted_at: Option<DateTime<Utc>>,
    pub _deleted_by: Option<Uuid>,
}

#[derive(Deserialize, Serialize)]
pub struct ClientData {
    pub client_id: Uuid,
    pub client_secret: String,
    pub client_name: String,
    pub redirect_uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct OauthTokenRequest {
    pub grant_type: String,
    pub refresh_token: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct MetaResponse {
    pub total_items: u16,
    pub items_per_page: u16,
    pub current_page: u16,
    pub prev_page: Option<u16>,
    pub next_page: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientResponse {
    pub data: Vec<ClientData>,
    pub meta: MetaResponse,
}

#[derive(Deserialize, Serialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub redirect: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateClientRequest {
    pub name: String,
    pub redirect: String,
}
