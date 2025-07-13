use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::web::ServiceConfig;
use actix_web::get;
use actix_web::web;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::Result;
use super::default_repository::Response;
use super::dotenv;
use askama::Template;
use async_std::path::PathBuf;
use serde::Serialize;
use std::env;

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/general/welcome.min.html")]
struct WelcomeTemplate {
    asset_url: String,
}

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/general/error-404.min.html")]
struct Error404Template {
    asset_url: String,
}

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/general/error-500.min.html")]
struct Error500Template {
    asset_url: String,
}

// Define a JSON response struct
#[allow(dead_code)]
#[derive(Serialize)]
struct MyData {
    message: &'static str,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(error_404_page)
            .service(error_500_page)
            .service(healthcheck)
            .service(healthz)
            .service(ready)
            .service(welcome_page);
    }
}

/// Healthcheck
///
/// Healthcheck
#[utoipa::path(
get,
path = "/v1/healthcheck",
responses(
(status = 200, description = "OK", content_type = "application/json", body = Response),
(status = 404, description = "NOT FOUND", content_type = "application/json", body = Response),
(status = 500, description = "INTERNAL SERVER ERROR", content_type = "application/json", body = Response)
),
tag = "Healthcheck"
)]
#[get("/v1/healthcheck")]
async fn healthcheck() -> Result<impl Responder, Error> {
    let obj = Response {
        data: None,
        error: None,
        status: StatusCode::OK.as_u16(),
        message: Some(StatusCode::OK.to_string()),
        meta: None,
    };
    Ok(web::Json(obj))
}

/// Healthz
///
/// Healthz
#[utoipa::path(
get,
path = "/healthz",
responses(
(status = 200, description = "OK", content_type = "application/json", body = Response),
(status = 404, description = "NOT FOUND", content_type = "application/json", body = Response),
(status = 500, description = "INTERNAL SERVER ERROR", content_type = "application/json", body = Response)
),
tag = "Kubernetes"
)]
#[get("/healthz")]
async fn healthz() -> Result<impl Responder, Error> {
    let obj = Response {
        data: None,
        error: None,
        status: StatusCode::OK.as_u16(),
        message: Some(StatusCode::OK.to_string()),
        meta: None,
    };
    Ok(web::Json(obj))
}

/// Ready
///
/// Ready
#[utoipa::path(
get,
path = "/ready",
responses(
(status = 200, description = "OK", content_type = "application/json", body = Response),
(status = 404, description = "NOT FOUND", content_type = "application/json", body = Response),
(status = 500, description = "INTERNAL SERVER ERROR", content_type = "application/json", body = Response)
),
tag = "Kubernetes"
)]
#[get("/ready")]
async fn ready() -> Result<impl Responder, Error> {
    let obj = Response {
        data: None,
        error: None,
        status: StatusCode::OK.as_u16(),
        message: Some(StatusCode::OK.to_string()),
        meta: None,
    };
    Ok(web::Json(obj))
}

#[get("/")]
async fn welcome_page() -> HttpResponse {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let _path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/general/coming-soon.min.html", template_type));

    let data = WelcomeTemplate {
        asset_url: dotenv::get_asset_url(),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[get("/error/404")]
async fn error_404_page(req: HttpRequest) -> Result<HttpResponse, Error> {
    handle_400(req).await
}

#[get("/error/500")]
async fn error_500_page() -> Result<HttpResponse, Error> {
    let data: Error500Template = Error500Template {
        asset_url: dotenv::get_asset_url(),
    };

    let rendered: String = data.render().unwrap();
    
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn handle_400(req: HttpRequest) -> Result<HttpResponse, Error> {
    // Check the client's accept header
    let accept = req.headers().get("accept").and_then(|v| v.to_str().ok());

    // Return JSON response if client prefers JSON
    if let Some(accept) = accept {
        if accept.contains("application/json") {
            // Create JSON response
            let resp = Response {
                data: None,
                error: None,
                message: Some("Resource not found".to_string()),
                meta: None,
                status: StatusCode::NOT_FOUND.as_u16(),
            };
        
            return Ok(HttpResponse::NotFound().json(resp));
        }
    }

    // Ok(HttpResponse::BadRequest().body("Bad request"))
    let data: Error404Template = Error404Template {
        asset_url: dotenv::get_asset_url(),
    };

    let rendered: String = data.render().unwrap();
    
    Ok(HttpResponse::Ok().body(rendered))
}