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
use super::default_repository::Response;
use super::dotenv;
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

pub fn configure(config: &mut ServiceConfig) {
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
        .service(welcome_page)
        .service(robots_txt_page);
}

/// Healthcheck
///
/// To get healthcheck status
#[utoipa::path(
    get,
    path = "/v1/healthcheck",
    params(),
    responses(
        (status = 100, description = "Continue", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
        (status = 101, description = "Switching Protocols", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
        (status = 103, description = "Early Hints", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
        (status = 200, description = "OK", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
        (status = 201, description = "Created", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
        (status = 202, description = "Accepted", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
        (status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
        (status = 204, description = "No Content", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
        (status = 205, description = "Reset Content", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
        (status = 206, description = "Partial Content", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
        (status = 300, description = "Multiple Choices", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
        (status = 301, description = "Moved Permanently", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
        (status = 302, description = "Found", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
        (status = 303, description = "See Other", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
        (status = 304, description = "Not Modified", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
        (status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
        (status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
        (status = 400, description = "Bad Request", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
        (status = 401, description = "Unauthorized", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
        (status = 402, description = "Payment Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
        (status = 403, description = "Forbidden", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
        (status = 404, description = "Not Found", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
        (status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
        (status = 406, description = "Not Acceptable", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
        (status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
        (status = 408, description = "Request Timeout", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
        (status = 409, description = "Conflict", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
        (status = 410, description = "Gone", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
        (status = 411, description = "Length Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
        (status = 412, description = "Precondition Failed", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
        (status = 413, description = "Payload Too Large", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
        (status = 414, description = "URI Too Long", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
        (status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
        (status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
        (status = 417, description = "Expectation Failed", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
        (status = 418, description = "I'm a teapot", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
        (status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
        (status = 425, description = "Too Early", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": "425 Too Early", "message": ""})),
        (status = 426, description = "Upgrade Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
        (status = 428, description = "Precondition Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
        (status = 429, description = "Too Many Requests", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
        (status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
        (status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
        (status = 500, description = "Internal Server Error", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
        (status = 501, description = "Not Implemented", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
        (status = 502, description = "Bad Gateway", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
        (status = 503, description = "Service Unavailable", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
        (status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
        (status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
        (status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
        (status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
        (status = 508, description = "Loop Detected", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
        (status = 510, description = "Not Extended", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
        (status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response, content_type = ["application/json", "application/xml"],
            example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
    ),
    security(
        (),
        ("my_auth" = ["read:items", "edit:items"]),
        ("token_jwt" = [])
    ),
    tag = "Default"
)]
#[get("/v1/healthcheck")]
async fn healthcheck() -> impl Responder {
    let resp = Response {
        data: None,
        error: None,
        status: StatusCode::OK.as_u16(),
        message: Some(StatusCode::OK.to_string()),
        meta: None,
    };
    
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&resp).unwrap());
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

#[get("/robots.txt")]
async fn robots_txt_page(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = PathBuf::from("static/robots.txt");

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