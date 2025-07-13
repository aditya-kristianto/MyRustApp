use std::env;
use tokio_postgres::Client;
use tokio_postgres::Error;
use tokio_postgres::NoTls;

#[allow(dead_code)]
pub async fn init(app_name: &str) -> Result<Client, Error> {
    println!("Init Postgres Database ...");

    let (client, connection) = tokio_postgres::connect(get_uri(app_name), NoTls)
        .await
        .unwrap_or_else(|error| {
            panic!(
                "Failed to connect to the database at {}:{}. Error: {}",
                get_host(),
                get_port(),
                error
            )
        });

    // Spawn a task to process the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!(
                "Failed to connect to the database at {}:{}. Error: {}",
                get_host(),
                get_port(),
                e
            );
        }
    });

    Ok(client)
}

#[allow(dead_code)]
pub fn get_uri(app_name: &str) -> &'static str {
    let env_var_name = format!("APP_{}_POSTGRES_URI", app_name.to_uppercase());
    let uri_string = env::var(&env_var_name)
        .expect(&format!("{} not found in the environment", env_var_name));
    let uri_str: &'static str = Box::leak(uri_string.into_boxed_str());
    
    uri_str
}

#[allow(dead_code)]
pub fn get_host() -> &'static str {
    let host_string =
        env::var("POSTGRES_HOST").expect("POSTGRES_HOST not found in the environment");
    let host_str: &'static str = Box::leak(host_string.into_boxed_str());

    host_str
}

pub fn get_port() -> &'static str {
    let port_string =
        env::var("POSTGRES_PORT").expect("POSTGRES_PORT not found in the environment");
    let port_str: &'static str = Box::leak(port_string.into_boxed_str());

    port_str
}
