use actix_files::NamedFile;
use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::ServiceConfig;
use actix_web::get;
use actix_web::web;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use askama::Template;
use async_std::path::PathBuf;
use std::env;

#[path = "../../pkg/dotenv/dotenv.rs"] mod dotenv;
#[path = "../repositories/repository.rs"] mod repository;

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/general/welcome.min.html")]
struct WelcomeTemplate {
    asset_url: String,
}

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(coming_soon)
            .service(contact_us_page)
            .service(healthcheck)
            .service(healthz)
            .service(ready)
            .service(dist)
            .service(plans_page)
            .service(verify_email)
            .service(terms_page)
            .service(welcome_page);
    }
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
    let obj = repository::Response {
        status: 200,
        message: StatusCode::OK.to_string(),
        meta: None,
    };
    Ok(web::Json(obj))
}

#[get("/assets/{filename:.*}")]
async fn asset(req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let filename = req.match_info().query("filename");
    let path: PathBuf = PathBuf::from(format!("templates/{}/src/{}", template_type, filename));

    Ok(NamedFile::open(path)?)
}

#[get("/dist/{filename:.*}")]
async fn dist(req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/{}", template_type, req.match_info().query("filename")));

    Ok(NamedFile::open(path)?)
}

#[get("/coming-soon")]
async fn coming_soon(_req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/general/coming-soon.min.html", template_type));

    Ok(NamedFile::open(path)?)
}

#[get("/verify-email")]
async fn verify_email(_req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/general/verify-email.min.html", template_type));

    Ok(NamedFile::open(path)?)
}

#[get("/welcome")]
async fn welcome_page(_req: HttpRequest) -> HttpResponse {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let _path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/general/welcome.min.html", template_type));

    let data = WelcomeTemplate {
        asset_url: dotenv::get_asset_url(),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[get("/terms")]
async fn terms_page(_req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/pages/team.min.html", template_type));

    Ok(NamedFile::open(path)?)
}

#[get("/plans")]
async fn plans_page(_req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/pages/pricing/table.min.html", template_type));

    Ok(NamedFile::open(path)?)
}

#[get("/contact-us")]
async fn contact_us_page(_req: HttpRequest) -> Result<NamedFile, Error> {
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/{}/dist/pages/contact.min.html", template_type));

    Ok(NamedFile::open(path)?)
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
                status: StatusCode::NOT_FOUND.to_string(),
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