extern crate http;

use actix_web::web::ServiceConfig;
use actix_web::Responder;
use actix_web::{get};

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(error_page_404)
            .service(error_page_500);
    }
}

/// Error Page
///
/// Error Page
#[get("/error/404")]
pub async fn error_page_404(email: String, password: String) -> impl Responder {
    format!("hello from get users");
    format!("{} {}", email, password)
}

/// Error Page
///
/// Error Page
#[get("/error/500")]
pub async fn error_page_500(email: String, password: String) -> impl Responder {
    format!("hello from get users");
    format!("{} {}", email, password)
}