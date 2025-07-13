use actix_web::{http::StatusCode, App, HttpServer, HttpRequest};
use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::web::{self, Data};
use employee::default_repository::CustomNaiveDate::CustomDate;
use employee::default_repository::DataValue;
use employee::default_repository::Error;
use employee::default_repository::Meta;
use employee::default_repository::Response;
use employee::dotenv;
use employee::middleware;
use employee::postgres;
use employee::repository::Employee::EmployeeDivision;
use employee::repository::Employee::EmployeeDepartment;
use employee::repository::Employee::EmployeeDirectorate;
use employee::repository::Employee::EmployeePosition;
use employee::repository::Employee::Employee;
use employee::repository::Employee::EmployeeProject;
use employee::swagger;
use std::env;
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../../modules/api/employee/mod.rs"] pub mod employee;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "My Rust Employee API",
    ),
    paths(
        employee::route::healthcheck,
        employee::v1::get_employee,
        employee::v1::create_employee,
        employee::v1::delete_employee,
        employee::v1::update_employee,
    ), components(schemas(
        CustomDate,
        DataValue,
        EmployeeDivision,
        EmployeeDepartment,
        EmployeeDirectorate,
        EmployeePosition,
        Employee,
        EmployeeProject,
        Error,
        Meta,
        Response,
    )
), 
modifiers(&SecurityAddon),
tags(
    (
        name = "Employee",
        description = "Employee API",
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

    let (http_host, http_port) = dotenv::get_http_host_and_port("employee");
    let app_worker_count = dotenv::get_app_worker_count();
    
    // Generate the OpenAPI spec
    let mut openapi = ApiDoc::openapi();

    let _result = swagger::init(&mut openapi).await;
    let postgres_client_result = postgres::init("employee").await;
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

    employee::logger::http_start(http_host.as_str(), http_port).await;
    employee::logger::tcp_start(http_host.as_str(), http_port).await;
    
    let postgres_client_arc = Arc::clone(&postgres_client);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres_client_arc.clone()))
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", openapi.clone()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    employee::default_route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
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
            .configure(employee::default_route::configure())
            .configure(employee::route::configure())
            .configure(employee::configure())
    })
    .workers(app_worker_count)
    .bind((http_host.clone(), http_port))?
    .run()
    .await?;

    Ok(())
}