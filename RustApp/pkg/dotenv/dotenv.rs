use dotenv::dotenv;
use std::env;

#[allow(dead_code)]
pub async fn init() {
    // Load environment variables from the .env file
    match dotenv() {
        Ok(_) => {
            println!("Environment variables loaded successfully.");
        }
        Err(e) => {
            eprintln!("Failed to load environment variables: {:?}", e);

            // Exit the application with a specific exit code (0 for success, non-zero for error).
            std::process::exit(1);
        }
    }
}

#[allow(dead_code)]
pub fn get_http_host_and_port(app_name: &str) -> (String, u16) {
    let http_host: String = env::var("APP_HTTP_HOST").expect("APP_HTTP_HOST not found in the environment");
    let env_var_name = format!("APP_{}_HTTP_PORT", app_name.to_uppercase());
    
    let http_port = env::var(&env_var_name)
        .expect(&format!("{} not found in the environment", env_var_name));

    let http_port_ref: u16 = http_port.parse().unwrap();

    (http_host, http_port_ref)
}

#[allow(dead_code)]
pub fn get_tcp_host_and_port() -> (String, u16) {
    let tcp_host = env::var("APP_TCP_HOST").expect("APP_TCP_HOST not found in the environment");
    let tcp_port = env::var("APP_TCP_PORT").expect("APP_TCP_PORT not found in the environment");

    let tcp_port_ref: u16 = tcp_port.parse().unwrap();

    (tcp_host, tcp_port_ref)
}

#[allow(dead_code)]
pub fn get_app_version() -> String {
    let app_version = env::var("APP_VERSION").expect("APP_VERSION not found in the environment");

    app_version
}

#[allow(dead_code)]
pub fn get_asset_url() -> String {
    let asset_url = env::var("ASSET_URL").expect("ASSET_URL not found in the environment");

    asset_url
}

#[allow(dead_code)]
pub fn get_allowed_origin() -> &'static str {
    let allowed_origin_string = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN not found in the environment");
    let allowed_origin_str: &'static str = Box::leak(allowed_origin_string.into_boxed_str());

    allowed_origin_str
}

#[allow(dead_code)]
pub fn get_app_worker_count() -> usize {
    // Retrieve the environment variable or provide a default value
    let app_worker_count_str = env::var("APP_WORKER_COUNT").unwrap_or_else(|_| "1".to_string());

    // Parse the string to u16, defaulting to 1 if parsing fails
    match app_worker_count_str.parse::<usize>() {
        Ok(count) => count,
        Err(_) => 1,
    }
}