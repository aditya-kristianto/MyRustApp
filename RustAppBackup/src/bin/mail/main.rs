use actix_web::{http::StatusCode, App, HttpServer, HttpRequest};
use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::web::{self, Data};
use mail::default_repository::DataValue;
use mail::default_repository::Error;
use mail::default_repository::Meta;
use mail::default_repository::Response;
use mail::dotenv;
use mail::middleware;
use mail::postgres;
use mail::repository::Mail::MailHistory;
use mail::swagger;
use std::env;
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../../modules/api/mail/mod.rs"] pub mod mail;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "My Rust Mail API",
    ),
    paths(
        mail::route::healthcheck,
        mail::v1::get_mail_histories,
    ), components(schemas(
        DataValue,
        MailHistory,
        Error,
        Meta,
        Response,
    )
), 
modifiers(&SecurityAddon),
tags(
    (
        name = "Mail",
        description = "Mail API",
        external_docs(url = "https://adityakristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut();
        if ! components.is_none() {
            let component = components.unwrap(); // we can unwrap safely since there already is components registered.
            component.add_security_scheme(
                "apiKey",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(middleware::_X_API_KEY_HEADER))),
            );
            component.add_security_scheme(
                "oauth2",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(actix_web::http::header::AUTHORIZATION.to_string())))
            );
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::init().await;

    let template_type = env::var("TEMPLATE_TYPE").expect("");
    
    // Set the ASKAMA_TEMPLATE_DIR environment variable
    env::set_var("ASKAMA_TEMPLATE_DIR", format!("templates/{}/dist/", template_type));

    let (http_host, http_port) = dotenv::get_http_host_and_port("mail");
    let app_worker_count = dotenv::get_app_worker_count();
    
    // Generate the OpenAPI spec
    let mut openapi = ApiDoc::openapi();

    let _result = swagger::init(&mut openapi).await;
    let postgres_client_result = postgres::init("mail").await;
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

    mail::logger::http_start(http_host.as_str(), http_port).await;
    mail::logger::tcp_start(http_host.as_str(), http_port).await;
    
    let postgres_client_arc = Arc::clone(&postgres_client);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres_client_arc.clone()))
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", openapi.clone()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    mail::default_route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
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
            .wrap(Compress::default())
            .configure(mail::default_route::configure())
            .configure(mail::route::configure())
            .configure(mail::configure())
    })
    .workers(app_worker_count)
    .bind((http_host.clone(), http_port))?
    .run()
    .await?;

    Ok(())
}