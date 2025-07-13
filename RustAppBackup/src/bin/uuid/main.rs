#[macro_use]
extern crate dotenv_codegen;
extern crate wrap;


use actix_cors::Cors;
// use actix_ratelimit::{MemoryStore};
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, ErrorHandlers, Logger};
use actix_web::{http::StatusCode, web::Data, App, HttpServer};
// use mongodb::{bson::doc, options::ClientOptions, Client};
use repository::uuid_repo::UUIDRepo;
// use utoipa::openapi::security::{SecurityScheme, ApiKey, ApiKeyValue};
use utoipa::OpenApi;
    // , Modify};
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod models;
mod repository;
mod service;
#[path = "../../../pkg/logger/logger.rs"] mod logger;
#[path = "../../../pkg/dotenv/dotenv.rs"] mod dotenv;
#[path = "../../middleware/middleware.rs"] mod middleware;

#[derive(OpenApi)]
#[openapi(
    info(description = "My Api description"),
    paths(
        // api::default_api::health_checker_handler,
        api::default_api::index_handler,
        api::uuid_api::get_duuid,
        api::uuid_api::get_new_uuid,
    ), components(schemas(
        models::default_model::Error,
        models::default_model::Meta,
        models::default_model::Response,
        models::uuid_model::DUUID,
        models::uuid_model::Request,
    )), 
    // modifiers(&SecurityAddon),
    tags(
    (
        name = "DUUID",
        description = "Generate Device Unique ID",
        external_docs(url = "https://adityakristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;
// struct SecurityAddon;

// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
//         components.add_security_scheme(
//             "apiKey",
//             SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(middleware::_X_API_KEY_HEADER))),
//         );
//         components.add_security_scheme(
//             "oauth2",
//             SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(actix_web::http::header::AUTHORIZATION.to_string())))
//         );
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (app_host, app_port) = dotenv::get_http_host_and_port("uuid");

    let app_name: &str = dotenv!("APP_NAME");
    let app_version: &str = dotenv!("APP_VERSION");
    // Allow bursts with up to five requests per IP address
    // and replenishes one element every two seconds
    // let governor_conf = GovernorConfigBuilder::default()
    //     .per_second(2)
    //     .burst_size(5)
    //     .finish()
    //     .unwrap();

    // let _ = init_mongodb().await;
    let db = UUIDRepo::init().await;
    let db_data = Data::new(db);
    
    // let _ = init_swagger().await;
    // let _store = MemoryStore::new();

    logger::http_start(app_host.as_str(), app_port).await;

    HttpServer::new(move || {
        // let auth = HttpAuthentication::basic(middleware::validator);
        
        App::new()
            .app_data(db_data.clone())
            .service(SwaggerUi::new("/swagger/{_:.*}")
            .url(format!("/docs/{}/swagger.json", app_name).to_string(), ApiDoc::openapi()))
            // .wrap(auth)
            .wrap(
                Cors::default()
                    // .allowed_origin(&format!("http://{}{}", app_host, app_port).to_string())
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]),
            )
            // .wrap(Governor::new(&governor_conf))
            // .wrap(HttpAuthentication::bearer(middleware::ok_validator))
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
            .wrap(DefaultHeaders::new().add(("X-Version", app_version)))
            // .wrap(RateLimiter::new(
            //     MemoryStoreActor::from(store.clone()).start())
            //         .with_interval(Duration::from_secs(60))
            //         .with_max_requests(100)
            // )
            .configure(api::uuid_api::configure())
            .configure(api::default_api::configure())
    })
    .bind((app_host, app_port))?
    .run()
    .await
}

// async fn init_mongodb() -> mongodb::error::Result<()> {
//     println!("Init MongoDB");
    
//     // Replace the placeholder with your Atlas connection string
//     let uri = dotenv!("MONGODB_URI");
//     let client_options =
//         ClientOptions::parse(uri)
//             .await?;
    
//     // Create a new client and connect to the server
//     let client = Client::with_options(client_options)?;
    
//     // Send a ping to confirm a successful connection
//     client
//         .database("admin")
//         .run_command(doc! {"ping": 1}, None)
//         .await.expect("Some error message");
    
//     println!("Successfully connected to MongoDB");
    
//     Ok(())
// }

// pub async fn init_swagger() -> std::io::Result<()> {
//     println!("Init Swagger");
    
//     let swagger_directory_path = format!("docs/{}", dotenv!("APP_NAME")).to_string();
//     let swagger_file_path = format!("docs/{}/swagger.json", dotenv!("APP_NAME")).to_string();
    
//     fs::create_dir_all(swagger_directory_path)?;
//     let mut file = File::create(swagger_file_path)?;
//     let swagger_json = ApiDoc::openapi().to_pretty_json()?;
    
//     file.write_all(swagger_json.as_bytes())?;

//     println!("Successfully init Swagger");
    
//     Ok(())
// }
