extern crate http;

use actix_files::NamedFile;
use actix_http::StatusCode;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{get, post};
use actix_web::web;
use actix_web::web::Query;
use actix_web::web::Redirect;
use actix_web::web::ServiceConfig;
use askama::Template;
use async_std::path::PathBuf;
use html_escape::{decode_html_entities, encode_text};
use serde::Deserialize;
use std::collections::HashMap;
use oauth2::{basic::BasicClient};
use oauth2::{
    CsrfToken, PkceCodeChallenge,
    Scope,
};
use std::env;
use std::fs;

#[path = "../../../../pkg/response/response.rs"] mod response;

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/layouts/corporate/sign-in.min.html")]
struct SignInTemplate {
    asset_url: String,
    base_url: String,
    sign_in_url: String,
}

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/layouts/corporate/reset-password.min.html")]
struct ForgotPasswordTemplate {
    asset_url: String,
    base_url: String,
}

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/layouts/corporate/new-password.min.html")]
struct NewPasswordTemplate {
    asset_url: String,
    base_url: String,
}

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "authentication/layouts/corporate/new-password.min.html")]
struct SignUpTemplate {
    asset_url: String,
    base_url: String,
}

#[derive(Deserialize)]
struct SignInRequest {
    email: String,
    password: String,
}

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(forgot_password_page)
            .service(new_password)
            .service(sign_in_google)
            .service(sign_in_google_callback)
            .service(sign_in_page)
            .service(sign_out)
            .service(sign_up_page)
            .service(verify_email);
    }
}

#[get("/api/sessions/oauth/google")]
pub async fn oauth_google(req: HttpRequest) -> Result<NamedFile, Error> {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let _state_param = params.get("state");
    let _code_param = params.get("code");
    let _scope_param = params.get("scope");
    let _authuser_param = params.get("authuser");
    let _prompt_param = params.get("prompt");

    let _template_type = env::var("TEMPLATE_TYPE").expect("");
    let path: PathBuf = PathBuf::from(format!("templates/authentication/layouts/{}/sign-in.min.html", "corporate"));

    Ok(NamedFile::open(path)?)
}

#[get("/sign-in")]
pub async fn sign_in_page(req: HttpRequest) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let layout_param = params.get("layout");
    let layout_value;
    match layout_param {
        Some(inner) => layout_value = check_layout(inner.as_str()).unwrap(),
        None        => layout_value = check_layout("").unwrap(),
    }
    let _template_type = env::var("TEMPLATE_TYPE").expect("");
    let _path: PathBuf = PathBuf::from(format!("templates/authentication/layouts/{}/sign-in.min.html", layout_value));
    let mut _html_content = "".to_string();
    let file_path = format!("templates/authentication/layouts/{}/sign-in.min.html", layout_value);

    match read_html_file(&file_path) {
        Ok(content) => {
            _html_content = content;
        }
        Err(err) => {
            eprintln!("Error reading the HTML file: {:?}", err);
        }
    };

    let app_http_host = env::var("APP_HTTP_HOST").expect("APP_HTTP_HOST not found in the environment");
    let app_web_http_port = env::var("APP_WEB_HTTP_PORT").expect("APP_WEB_HTTP_PORT not found in the environment");

    let data = SignInTemplate {
        base_url: format!("http://{}:{}", app_http_host, app_web_http_port),
        asset_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
        sign_in_url: env::var("SIGN_IN_URL").expect("SIGN_IN_URL not found in the environment")
    };

    let rendered = data.render().expect("Failed to render the template");

    HttpResponse::Ok().body(rendered)
}

#[get("/sign-out")]
pub async fn sign_out(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Found()
        .append_header(("location", "/sign-in"))
        .finish()
}

fn read_html_file(file_path: &str) -> Result<String, std::io::Error> {
    // Read the HTML file into a string
    let html_content = fs::read_to_string(file_path)?;

    // Convert HTML symbols to their corresponding characters
    let decoded_html = decode_html_entities(&html_content);

    // Print the decoded HTML
    // println!("{}", decoded_html.to_string());

    // You can also encode HTML entities if needed
    let _encoded_html = encode_text(&decoded_html);

    // Print the encoded HTML
    // println!("{}", encoded_html);

    Ok(decoded_html.to_string())
}

#[get("/sign-in/google")]
pub async fn sign_in_google(client: web::Data<BasicClient>) -> HttpResponse {
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the OAuth2 authorization URL and redirect to it
    let (auth_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!("location : {}", auth_url.to_string());

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/sign-in/google/callback")]
pub async fn sign_in_google_callback() -> impl Responder {
    Redirect::to("/dashboard").permanent()
}

#[get("/sign-up")]
pub async fn sign_up_page(req: HttpRequest) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let layout_param = params.get("layout");
    let layout_value;
    match layout_param {
        Some(inner) => layout_value = check_layout(inner.as_str()).unwrap(),
        None        => layout_value = check_layout("").unwrap(),
    }
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let _path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/layouts/{}/sign-up.min.html", template_type, layout_value));

    // Ok(NamedFile::open(path)?)

    let data = SignUpTemplate {
        asset_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
        base_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)

    // Initialize Tera
    // let mut tera = Tera::default();
    //
    // // Load the template from a file
    // tera.add_template_file("templates/main_template.min.html", Some("main_template"))
    //     .expect("Failed to load template.");
    //
    // // Create a context with dynamic content
    // let mut context = Context::new();
    // context.insert("title", "Dynamic HTML Example");
    // context.insert("header", "Welcome to Rust Tera!");
    // context.insert("content", "This content is dynamic.");
    //
    // // Render the template with the dynamic context
    // let rendered_html = tera.render("main_template", &context)
    //     .expect("Failed to render template.");
}

fn check_layout(layout: &str) -> Result<&str, Error> {
    let mut default_layout = "corporate";

    if layout == "corporate" || layout == "creative" || layout == "fancy" || layout == "overlay" {
        default_layout = layout;
    }

    Ok(default_layout)
}

#[get("/forgot-password")]
pub async fn forgot_password_page(req: HttpRequest) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let layout_param = params.get("layout");
    let layout_value;
    match layout_param {
        Some(inner) => layout_value = check_layout(inner.as_str()).unwrap(),
        None        => layout_value = check_layout("").unwrap(),
    }
    let template_type = env::var("TEMPLATE_TYPE").expect("");
    let _path: PathBuf = PathBuf::from(format!("templates/{}/dist/authentication/layouts/{}/reset-password.min.html", template_type, layout_value));

    // Ok(NamedFile::open(path)?)
    let data = ForgotPasswordTemplate {
        asset_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
        base_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

#[get("/new-password")]
pub async fn new_password(req: HttpRequest) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let _layout_param = params.get("layout");
    let data = NewPasswordTemplate {
        asset_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
        base_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

/// Verify Email
///
/// Verify Email
#[utoipa::path(
post,
path = "/auth/email/verify",
responses(
(status = 100, description = "Continue", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONTINUE.to_string(), "message": ""})),
(status = 101, description = "Switching Protocols", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SWITCHING_PROTOCOLS.to_string(), "message": ""})),
(status = 103, description = "Early Hints", content_type = "application/json", body = Response,
example = json!({"status": "103 Early Hints".to_string(), "message": ""})),
(status = 200, description = "OK", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::OK.to_string(), "message": ""})),
(status = 201, description = "Created", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CREATED.to_string(), "message": ""})),
(status = 202, description = "Accepted", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::ACCEPTED.to_string(), "message": ""})),
(status = 203, description = "Non-Authoritative Information", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NON_AUTHORITATIVE_INFORMATION.to_string(), "message": ""})),
(status = 204, description = "No Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NO_CONTENT.to_string(), "message": ""})),
(status = 205, description = "Reset Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RESET_CONTENT.to_string(), "message": ""})),
(status = 206, description = "Partial Content", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PARTIAL_CONTENT.to_string(), "message": ""})),
(status = 300, description = "Multiple Choices", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MULTIPLE_CHOICES.to_string(), "message": ""})),
(status = 301, description = "Moved Permanently", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::MOVED_PERMANENTLY.to_string(), "message": ""})),
(status = 302, description = "Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FOUND.to_string(), "message": ""})),
(status = 303, description = "See Other", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SEE_OTHER.to_string(), "message": ""})),
(status = 304, description = "Not Modified", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_MODIFIED.to_string(), "message": ""})),
(status = 307, description = "Temporary Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TEMPORARY_REDIRECT.to_string(), "message": ""})),
(status = 308, description = "Permanent Redirect", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PERMANENT_REDIRECT.to_string(), "message": ""})),
(status = 400, description = "Bad Request", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_REQUEST.to_string(), "message": ""})),
(status = 401, description = "Unauthorized", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAUTHORIZED.to_string(), "message": ""})),
(status = 402, description = "Payment Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYMENT_REQUIRED.to_string(), "message": ""})),
(status = 403, description = "Forbidden", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::FORBIDDEN.to_string(), "message": ""})),
(status = 404, description = "Not Found", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_FOUND.to_string(), "message": ""})),
(status = 405, description = "Method Not Allowed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::METHOD_NOT_ALLOWED.to_string(), "message": ""})),
(status = 406, description = "Not Acceptable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_ACCEPTABLE.to_string(), "message": ""})),
(status = 407, description = "Proxy Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PROXY_AUTHENTICATION_REQUIRED.to_string(), "message": ""})),
(status = 408, description = "Request Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_TIMEOUT.to_string(), "message": ""})),
(status = 409, description = "Conflict", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::CONFLICT.to_string(), "message": ""})),
(status = 410, description = "Gone", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GONE.to_string(), "message": ""})),
(status = 411, description = "Length Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LENGTH_REQUIRED.to_string(), "message": ""})),
(status = 412, description = "Precondition Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_FAILED.to_string(), "message": ""})),
(status = 413, description = "Payload Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PAYLOAD_TOO_LARGE.to_string(), "message": ""})),
(status = 414, description = "URI Too Long", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::URI_TOO_LONG.to_string(), "message": ""})),
(status = 415, description = "Unsupported Media Type", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNSUPPORTED_MEDIA_TYPE.to_string(), "message": ""})),
(status = 416, description = "Range Not Satisfiable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::RANGE_NOT_SATISFIABLE.to_string(), "message": ""})),
(status = 417, description = "Expectation Failed", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::EXPECTATION_FAILED.to_string(), "message": ""})),
(status = 418, description = "I'm a teapot", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::IM_A_TEAPOT.to_string(), "message": ""})),
(status = 422, description = "Unprocessable Entity", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNPROCESSABLE_ENTITY.to_string(), "message": ""})),
(status = 425, description = "Too Early", content_type = "application/json", body = Response,
example = json!({"status": "425 Too Early", "message": ""})),
(status = 426, description = "Upgrade Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UPGRADE_REQUIRED.to_string(), "message": ""})),
(status = 428, description = "Precondition Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::PRECONDITION_REQUIRED.to_string(), "message": ""})),
(status = 429, description = "Too Many Requests", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::TOO_MANY_REQUESTS.to_string(), "message": ""})),
(status = 431, description = "Request Header Fields Too Large", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE.to_string(), "message": ""})),
(status = 451, description = "Unavailable For Legal Reasons", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS.to_string(), "message": ""})),
(status = 500, description = "Internal Server Error", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INTERNAL_SERVER_ERROR.to_string(), "message": ""})),
(status = 501, description = "Not Implemented", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_IMPLEMENTED.to_string(), "message": ""})),
(status = 502, description = "Bad Gateway", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::BAD_GATEWAY.to_string(), "message": ""})),
(status = 503, description = "Service Unavailable", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::SERVICE_UNAVAILABLE.to_string(), "message": ""})),
(status = 504, description = "Gateway Timeout", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::GATEWAY_TIMEOUT.to_string(), "message": ""})),
(status = 505, description = "HTTP Version Not Supported", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::HTTP_VERSION_NOT_SUPPORTED.to_string(), "message": ""})),
(status = 506, description = "Variant Also Negotiates", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::VARIANT_ALSO_NEGOTIATES.to_string(), "message": ""})),
(status = 507, description = "Insufficient Storage", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::INSUFFICIENT_STORAGE.to_string(), "message": ""})),
(status = 508, description = "Loop Detected", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::LOOP_DETECTED.to_string(), "message": ""})),
(status = 510, description = "Not Extended", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NOT_EXTENDED.to_string(), "message": ""})),
(status = 511, description = "Network Authentication Required", content_type = "application/json", body = Response,
example = json!({"status": StatusCode::NETWORK_AUTHENTICATION_REQUIRED.to_string(), "message": ""}))
),
request_body = Request,
tag = "Auth"
)]
#[post("/auth/email/verify")]
pub async fn verify_email(email: String, password: String) -> impl Responder {
    format!("hello from get users");
    format!("{} {}", email, password)
}