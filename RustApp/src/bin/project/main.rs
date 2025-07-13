use actix_web::{http::StatusCode, App, HttpServer, HttpRequest};
use actix_web::middleware::{Compress, ErrorHandlers, Logger};
use actix_web::web::{self, Data};
use project::default_repository::DataValue;
use project::default_repository::Error;
use project::default_repository::Header;
use project::default_repository::Meta;
use project::default_repository::QueryParams;
use project::default_repository::Response;
use project::dotenv;
use project::middleware;
use project::postgres;
use project::repository::Project::ProjectSatus;
use project::repository::Project::Project;
use project::repository::Project::ProjectApproval;
use project::repository::Project::ProjectTicketApproval;
use project::repository::Project::ProjectTicket;
use project::repository::Project::ProjectItem;
use project::repository::Project::ProjectRequest;
use project::repository::Project::ProjectSurveyQuestion;
use project::repository::Project::ProjectSurveyFeedback;
use project::repository::Project::ProjectSurveyAnswer;
use project::swagger;
use std::env;
use std::sync::Arc;
use tokio_postgres::Client;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa_swagger_ui::SwaggerUi;

#[path = "../../modules/api/project/mod.rs"] pub mod project;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "My Rust Project API",
    ),
    paths(
        project::route::healthcheck,
        project::v1::project::create_project,
        project::v1::project::delete_project,
        project::v1::project::get_projects,
        project::v1::project::update_project,
        project::v1::project_approval::create_project_approval,
        project::v1::project_approval::delete_project_approval,
        project::v1::project_approval::get_project_approval,
        project::v1::project_approval::update_project_approval,
        project::v1::project_item::create_project_item,
        project::v1::project_item::delete_project_item,
        project::v1::project_item::get_project_item,
        project::v1::project_item::update_project_item,
        project::v1::project_request::create_project_request,
        project::v1::project_request::delete_project_request,
        project::v1::project_request::get_project_request,
        project::v1::project_request::update_project_request,
        project::v1::project_status::create_project_status,
        project::v1::project_status::delete_project_status,
        project::v1::project_status::get_project_status,
        project::v1::project_status::update_project_status,
        project::v1::project_survey_answer::create_project_survey_answer,
        project::v1::project_survey_answer::delete_project_survey_answer,
        project::v1::project_survey_answer::get_project_survey_answer,
        project::v1::project_survey_answer::update_project_survey_answer,
        project::v1::project_survey_question::create_project_survey_question,
        project::v1::project_survey_question::delete_project_survey_question,
        project::v1::project_survey_question::get_project_survey_question,
        project::v1::project_survey_question::update_project_survey_question,
        project::v1::project_ticket::create_project_ticket,
        project::v1::project_ticket::delete_project_ticket,
        project::v1::project_ticket::get_project_ticket,
        project::v1::project_ticket::update_project_ticket,
        project::v1::project_ticket_approval::create_project_ticket_approval,
        project::v1::project_ticket_approval::delete_project_ticket_approval,
        project::v1::project_ticket_approval::get_project_ticket_approval,
        project::v1::project_ticket_approval::update_project_ticket_approval,
    ), components(schemas(
        DataValue,
        Error,
        Header,
        Meta,
        ProjectSatus,
        Project,
        ProjectApproval,
        ProjectTicketApproval,
        ProjectTicket,
        ProjectItem,
        ProjectRequest,
        ProjectSurveyQuestion,
        ProjectSurveyFeedback,
        ProjectSurveyAnswer,
        QueryParams,
        Response,
    )
), 
modifiers(&SecurityAddon),
tags(
    (
        name = "Project",
        description = "Project API",
        external_docs(url = "https://aditya-kristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://aditya-kristianto.com/", description = "Find out more"))]
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

    let (http_host, http_port) = dotenv::get_http_host_and_port("project");
    let app_worker_count = dotenv::get_app_worker_count();
    
    // Generate the OpenAPI spec
    let mut openapi = ApiDoc::openapi();

    let _result = swagger::init(&mut openapi).await;
    let postgres_client_result = postgres::init("project").await;
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

    project::logger::http_start(http_host.as_str(), http_port).await;
    project::logger::tcp_start(http_host.as_str(), http_port).await;
    
    let postgres_client_arc = Arc::clone(&postgres_client);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres_client_arc.clone()))
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", openapi.clone()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    project::default_route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
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
            .configure(project::default_route::configure())
            .configure(project::route::configure())
            .configure(project::configure())
    })
    .workers(app_worker_count)
    .bind((http_host.clone(), http_port))?
    .run()
    .await?;

    Ok(())
}