const _ENV_DEVELOPMENT: &str = "development";
const _ENV_LOCAL: &str = "local";
const _ENV_PRODUCTION: &str = "production";
const _ENV_STAGE: &str = "stage";
pub const _AUTHORIZATION_HEADER: &str = "Authorization";


pub fn _is_development() -> bool {
    let app_env = dotenv!("APP_ENV").to_lowercase();

    return app_env == _ENV_DEVELOPMENT
}

pub fn _is_local() -> bool {
    let app_env = dotenv!("APP_ENV").to_lowercase();

    return app_env == _ENV_LOCAL
}

pub fn _is_production() -> bool {
    let app_env = dotenv!("APP_ENV").to_lowercase();

    return app_env == _ENV_PRODUCTION
}

pub fn _is_stage() -> bool {
    let app_env = dotenv!("APP_ENV").to_lowercase();

    return app_env == _ENV_STAGE
}