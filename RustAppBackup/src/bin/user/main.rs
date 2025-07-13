use actix_cors::Cors;
use actix_http::StatusCode;
use actix_web::App;
use actix_web::middleware::DefaultHeaders;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::http::header;
use actix_web::http::header::HeaderName;
use actix_web::HttpServer;
use actix_web::web::Data;
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use user::default_repository::Response;
use user::default_repository::Meta;

// #[path = "../../db/connection/mongodb.rs"] mod mongodb;
#[path = "../../modules/api/middleware/middleware.rs"] mod middleware;
#[path = "../../modules/api/user/user.rs"] mod model;
#[path = "../../../pkg/dotenv/dotenv.rs"] mod dotenv;
#[path = "../../../pkg/db/postgres/postgres.rs"] mod postgres;
#[path = "../../../pkg/logger/logger.rs"] mod logger;
#[path = "../../../pkg/swagger/swagger.rs"] mod swagger;
#[path = "../../modules/api/user/mod.rs"] pub mod user;

#[derive(OpenApi)]
#[openapi(paths(
    user::v1::create_user,
    user::v1::delete_user,
    user::v1::select_user,
    user::v1::update_user,
), components(schemas(
    model::UserGetRequest,
    Response,
    Meta,
)),
tags(
(
    name = "User",
    description = "User API",
    external_docs(url = "https://adityakristianto.com/", description = "Find out more")
)
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (app_host, app_port) = dotenv::get_http_host_and_port("user");
    let app_version = dotenv::get_app_version();
    // let db = mongodb::DB::init().await;
    // if db.as_ref().is_err() {
    //     print!("{:?}", db.as_ref().err())
    // }

    // for db_name in db.unwrap().client.list_database_names(None, None).await {
    //     println!("{:?}", db_name);
    // }

    // while let Ok(db_name) = db.as_ref().unwrap().client.list_database_names(None, None).await {
    //     println!("{:?}", db_name);
    // }

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

    let _result = swagger::init(&mut ApiDoc::openapi()).await;
    let postgres_client_arc = Arc::clone(&postgres_client);

    logger::http_start(app_host.as_str(), app_port).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres_client_arc.clone()))
            .configure(user::configure())
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", ApiDoc::openapi()))
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
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                        HeaderName::from_static("x-api-key"),
                    ]),
            )
    })
    .workers(1)
    .bind((app_host, app_port))?
    .run()
    .await
}