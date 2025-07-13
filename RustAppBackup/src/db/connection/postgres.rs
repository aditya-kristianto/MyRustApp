use tokio_postgres::{NoTls, Client, Error};
use std::env;

pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        // Parse your connection string into an options struct
        let host = env::var("POSTGRES_HOST").expect("");
        let username = env::var("POSTGRES_USERNAME").expect("");
        let password = env::var("POSTGRES_PASSWORD").expect("");
        let database = env::var("POSTGRES_DATABASE").expect("");

        let (client, _connection) = tokio_postgres::connect(format!("host={} user={} password={} dbname={}", host, username, password, database).as_str(), NoTls)
            .await?;

        Ok(Self{
            client: client,
        })
    }
}
