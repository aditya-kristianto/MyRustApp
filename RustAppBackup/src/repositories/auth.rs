use std::string::String;
use utoipa::ToSchema;

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct RequestBody {
    // #[serde(default = "default_email")]
    #[schema(example = "kristianto.aditya@gmail.com")]
    pub email: String,
    // #[serde(default = "default_password")]
    #[schema(example = "password")]
    pub password: String,
}

#[derive(ToSchema)]
pub struct RequestHeader {
    // #[serde(default = "default_email")]
    #[schema(example = "ADR...")]
    pub _x_api_key: String,
}

fn _default_email() -> String {
    "kristianto.aditya@gmail.com".to_string()
}

fn _default_password() -> String {
    "password".to_string()
}