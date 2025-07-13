use actix_cors::Cors;
use actix_limitation::Limiter;
use actix_limitation::RateLimiter;
use actix::prelude::*;
use actix_session::SessionExt;
use actix_web::App;
use actix_web::web::Data;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::dev::ServiceRequest;
use actix_web::HttpServer;
use actix_web::middleware::DefaultHeaders;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::Result;
use actix_web::web;
use crate::repositories::default::Meta;
use crate::repositories::default::Response;
use std::time::Duration;
use std::env;
use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::RedirectUrl;
use oauth2::TokenUrl;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio_postgres::Client;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod repositories;
mod utils;
#[path = "../../repositories/auth.rs"] mod auth_repository;
#[path = "../../middleware/middleware.rs"] mod middleware;
#[path = "../../../pkg/dotenv/dotenv.rs"] mod dotenv;
#[path = "../../../pkg/logger/logger.rs"] mod logger;
#[path = "../../../pkg/db/postgres/postgres.rs"] mod postgres;
#[path = "../../../pkg/swagger/swagger.rs"] mod swagger;
#[path = "../../modules/api/auth/mod.rs"] pub mod auth;

#[derive(OpenApi)]
#[openapi(paths(
    auth::v1::sign_in,
    auth::v1::sign_out,
    auth::route::healthcheck,
    auth::route::healthz,
    auth::route::ready,
), components(schemas(
    auth_repository::RequestBody,
    auth_repository::RequestHeader,
    Response,
    Meta,
    // Pet
)), tags(
    (
        name = "DUUID",
        description = "Generate Device Unique ID",
        external_docs(url = "https://adityakristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;

/// Define a new actor for handling TCP connections
struct TcpServer;

impl Actor for TcpServer {
    type Context = Context<Self>;
}

/// Define a message type for TCP connections
struct TcpConnection(pub TcpStream);

impl Message for TcpConnection {
    type Result = ();
}

impl Handler<TcpConnection> for TcpServer {
    type Result = ();

    fn handle(&mut self, msg: TcpConnection, _: &mut Self::Context) {
        let mut stream = msg.0;
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer);
        let response = "Hello from TCP server!\n";
        let _ = stream.write_all(response.as_bytes());
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (http_host, http_port) = dotenv::get_http_host_and_port("auth");
    let app_version = dotenv::get_app_version();
    // Configure the OAuth2 client
    // Attempt to retrieve the Google Client ID from the environment variable
    // let client_id = match env::var("GOOGLE_CLIENT_ID") {
    //     Ok(value) => ClientId::new(value),
    //     Err(_) => {
    //         eprintln!("Error: GOOGLE_CLIENT_ID environment variable not set or is empty.");
    //         // Handle the error gracefully, such as returning a default value or terminating the application.
    //         std::process::exit(1);
    //     }
    // };
    // Attempt to retrieve the Google Client Secret from the environment variable
    // let client_secret = match env::var("GOOGLE_CLIENT_SECRET") {
    //     Ok(value) => ClientSecret::new(value),
    //     Err(_) => {
    //         eprintln!("Error: GOOGLE_CLIENT_ID environment variable not set or is empty.");
    //         // Handle the error gracefully, such as returning a default value or terminating the application.
    //         std::process::exit(1);
    //     }
    // };
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not found in the environment");
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not found in the environment");
    let client_id = ClientId::new(google_client_id);
    let client_secret = ClientSecret::new(google_client_secret);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).expect("Invalid token endpoint URL");

    // Create a RedirectUrl from your string
    let redirect_url = RedirectUrl::new("http://localhost:8080/sign-in/google/callback".to_string()).unwrap();

    let _client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
        .set_redirect_uri(redirect_url);

    let _swagger = swagger::init(&mut ApiDoc::openapi()).await;
    let postgres_client_result = postgres::init().await;
    let postgres_client: Arc<Client>;

    match postgres_client_result {
        Ok(client) => {
            println!("Connected to the database");
            postgres_client = Arc::new(client);
        }
        Err(e) => {
            eprintln!("Failed to connect to the database at {}:{}. Error: {}", postgres::get_host(), postgres::get_port(), e);

            panic!("Unable to initialize the database client");
        }
    }

    let postgres_client_arc = Arc::clone(&postgres_client);

    

    // Start Actix HTTP server
    HttpServer::new(move || {
        // let client = Arc::clone(&postgres_client);

        let _auth = HttpAuthentication::basic(|req, credentials| async move {
            let _credential = credentials.user_id();
            let _password;
            match credentials.password() {
                Some(inner) => _password = inner,
                None => todo!(),
            }

            Ok(req)
        });

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let limiter = web::Data::new(
            Limiter::builder("redis://127.0.0.1")
                .key_by(|req: &ServiceRequest| {
                    req.get_session()
                        .get(&"session-id")
                        .unwrap_or_else(|_| req.cookie(&"rate-api-id").map(|c| c.to_string()))
                })
                .limit(5000)
                .period(Duration::from_secs(3600)) // 60 minutes
                .build()
                .unwrap(),
        );

        App::new()
            // .app_data(Data::new(client.clone()))
            // .app_data(client.clone())
            // .data(Data::new(client.clone()))
            //.data(db.clone()) // Share the database connection across handlers
            //.route("/", web::get().to(handle_request))
            .wrap(cors)
            .app_data(limiter.clone())
            .app_data(Data::new(postgres_client_arc.clone()))
            .configure(auth::configure())
            .configure(auth::route::configure())
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", ApiDoc::openapi()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    auth::default_route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
                })
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::BAD_REQUEST,
                middleware::add_error_header,
            ))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::NOT_FOUND,
                middleware::add_error_header,
            ))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::INTERNAL_SERVER_ERROR,
                middleware::add_error_header,
            ))
            .wrap(DefaultHeaders::new().add(("X-Version", app_version.to_string())))
            .wrap(
                Cors::default()
                    .allowed_origin(dotenv::get_allowed_origin())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ]),
            )
            .wrap(RateLimiter::default())
    })
    .workers(1)
    .bind((http_host.clone(), http_port))?
    .run()
    .await?;

    logger::http_start(http_host.as_str(), http_port).await;

    // Start TCP server in a separate thread
    let tcp_listener = TcpListener::bind("127.0.0.1:9090").unwrap();
    thread::spawn(move || {
        for stream in tcp_listener.incoming() {
            let stream = stream.unwrap();
            // Get address of the TcpServer actor and send TcpConnection message
            let addr = TcpServer.start();
            addr.do_send(TcpConnection(stream));
        }
    });

    logger::tcp_start(http_host.as_str(), http_port).await;

    Ok(())
}

async fn _connect_to_database() -> Result<Arc<Mutex<Client>>, tokio_postgres::Error> {
    let db_url = env::var("POSTGRES_URI").expect("POSTGRES_URI not found in .env file");
    println!("db_url : {}", db_url);
    let (client, connection) = tokio_postgres::connect(
        &db_url,
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(client)))
}


async fn _establish_db_pool() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let db_url = env::var("POSTGRES_URI").expect("POSTGRES_URI not found in .env file");
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await.expect("Failed to connect to the database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    Ok(client)
}