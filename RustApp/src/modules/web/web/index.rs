extern crate http;

use actix_service::Service;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::{HttpResponse, Result};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;

#[path = "../../../repositories/duuid.rs"]
mod repository;

pub(super) fn configure(config: &mut ServiceConfig) {
    config.service(home_page);
}

#[get("/")]
pub async fn home_page(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
        .append_header(("Location", "/welcome"))
        .status(StatusCode::PERMANENT_REDIRECT)
        .finish())
}