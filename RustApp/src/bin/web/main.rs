extern crate http;

use actix_cors::Cors;
use actix_web::App;
use actix_web::web::Data;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::HttpServer;
use actix_web::middleware::Compress;
use actix_web::middleware::DefaultHeaders;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;
use std::env;
use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::RedirectUrl;
use oauth2::TokenUrl;
use tera::Tera;

mod models;
#[path = "../../modules/web/auth/auth.rs"] mod auth;
#[path = "../../repositories/auth.rs"] mod auth_repository;
#[path = "../../dashboard/dashboard.rs"] mod dashboard;
#[path = "../../middleware/middleware.rs"] mod middleware;
#[path = "../../repositories/repository.rs"] mod repository;
#[path = "../../routes/route.rs"] mod route;
#[path = "../../modules/web/web/index.rs"] mod web;
#[path = "../../../pkg/logger/logger.rs"] mod logger;
#[path = "../../../pkg/dotenv/dotenv.rs"] mod dotenv;



#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;
    
    // Load environment variables from the .env file
    // match dotenv() {
    //     Ok(_) => {
    //         println!("Environment variables loaded successfully.");
    //         // At this point, your environment variables are available to use.
    //         // You can access them using std::env::var("VAR_NAME").
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to load environment variables: {:?}", e);
    //         // Handle the error or exit your application.
    //         // Exit the application with a specific exit code (0 for success, non-zero for error).
    //         std::process::exit(1);
    //     }
    // }

    // Loop until the environment variable you're waiting for is loaded.
    // let mut retries = 0;
    // while env::var("GOOGLE_CLIENT_ID").is_err() {
    //     if retries >= 30 {
    //         eprintln!("Timed out waiting for GOOGLE_CLIENT_ID to be set.");
    //         std::process::exit(1);
    //     }
    //
    //     eprintln!("Waiting for YOUR_ENV_VARIABLE to be set...");
    //     thread::sleep(Duration::from_secs(1));
    //     retries += 1;
    // }
    
    let (app_host, app_port) = dotenv::get_http_host_and_port("web");
    
    // Configure the OAuth2 client
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not found in the environment").to_string());
    let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not found in the environment").to_string());
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).expect("Invalid token endpoint URL");

    // Create a RedirectUrl from your string
    let redirect_url = RedirectUrl::new("http://localhost:8080/sign-in/google/callback".to_string()).unwrap();

    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url);

    let tera = Tera::new("templates2/*.min.html").expect("Error parsing templates");

    logger::http_start(app_host.as_str(), app_port).await;

    // Start the Actix web application
    HttpServer::new(move || {
        let app_version = env::var("APP_VERSION").expect("APP_VERSION not found in the environment");

        let _auth = HttpAuthentication::basic(|req, credentials| async move {
            let _credential = credentials.user_id();
            let _password;
            match credentials.password() {
                Some(inner) => _password = inner,
                None => todo!(),
            }

            Ok(req)
        });

        App::new()
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(tera.clone()))
            //.data(db.clone()) // Share the database connection across handlers
            .configure(web::configure())
            .configure(auth::configure())
            .configure(dashboard::configure())
            .configure(route::configure())
            .default_service(
                actix_web::web::route().to(|req: HttpRequest| async {
                    route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
                })
            )
            .wrap(middleware::ContentSecurityPolicy)
            .wrap(Compress::default()) // This ensures gzip compression is applied
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("https://assets.aditya-kristianto.com")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, middleware::add_error_header),
            )
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::NOT_FOUND, middleware::add_error_header),
            )
            .wrap(DefaultHeaders::new().add(("X-Version", app_version.to_string())))
    })
        .workers(1)
        .bind((app_host, app_port))?
        .run()
        .await?;

    Ok(()) // R
}