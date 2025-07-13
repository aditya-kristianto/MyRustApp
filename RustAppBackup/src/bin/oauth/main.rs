use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::HttpRequest;
use actix_web::{http::StatusCode, App, HttpServer};
use actix_web::web;
use utoipa::openapi::security::{SecurityScheme, ApiKey, ApiKeyValue};
use utoipa::{OpenApi, Modify};
use utoipa_swagger_ui::SwaggerUi;
use oauth::repository::Meta;
use oauth::dotenv;
use oauth::swagger;

mod models;
#[path = "../../middleware/middleware.rs"] mod middleware;
#[path = "../../repositories/oauth.rs"] mod repository;
#[path = "../../../pkg/logger/logger.rs"] mod logger;
#[path = "../../modules/api/oauth/mod.rs"]
pub mod oauth;

#[derive(OpenApi)]
#[openapi(paths(
    // authorize::authorize,
    // authorize::approve,
    // authorize::deny,
    // clients::for_user,
    // clients::store,
    // clients::update,
    // clients::destroy,
    // personal_access_tokens::for_user,
    // personal_access_tokens::store,
    // personal_access_tokens::destroy,
    // scopes::all,
    // tokens::issue_token,
    // tokens::refresh,
    // tokens::for_user, 
    // tokens::destroy
), components(schemas(
    oauth::repository::OauthAccessTokens,
    oauth::repository::OauthAuthCodes,
    oauth::repository::OauthClients,
    oauth::repository::OauthPersonalAccessClients,
    oauth::repository::OauthRefreshTokens,
    oauth::repository::PasswordResets,
    oauth::repository::PersonalAccessTokens,
    oauth::repository::Response,
    Meta,
)), 
modifiers(&SecurityAddon),
tags(
    (
        name = "OAuth2",
        description = "Open Authentication",
        external_docs(url = "https://adityakristianto.com/", description = "Find out more")
    )
), external_docs(url = "https://adityakristianto.com/", description = "Find out more"))]
struct ApiDoc;
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "apiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(middleware::_X_API_KEY_HEADER))),
        );
        components.add_security_scheme(
            "oauth2",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(actix_web::http::header::AUTHORIZATION.to_string())))
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (app_host, app_port) = dotenv::get_http_host_and_port("oauth");
    let _result = swagger::init(&mut ApiDoc::openapi()).await;

    logger::http_start(app_host.as_str(), app_port).await;

    HttpServer::new(move || {
        App::new()
            .service(swagger::get_swagger_json)
            .service(SwaggerUi::new("/swagger/{_:.*}").url("/docs/swagger.json", ApiDoc::openapi()))
            .default_service(
                web::route().to(|req: HttpRequest| async {
                    oauth::route::handle_400(req).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))
                })
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(ErrorHandlers::new().handler(
                StatusCode::INTERNAL_SERVER_ERROR,
                middleware::add_error_header,
            ))
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, middleware::add_error_header))
            .configure(oauth::configure())
            .configure(oauth::route::configure())
    })
    .bind((app_host, app_port))?
    .run()
    .await
}