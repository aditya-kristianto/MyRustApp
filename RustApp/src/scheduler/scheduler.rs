use actix_rt::time;
use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::ServiceConfig;
use std::time::Duration;

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(scheduler);
    }
}

/// Scheduler
///
/// Scheduler 
#[utoipa::path(
    get,
    path = "/scheduler",
    responses(
        (status = 200, description = "OK", body = String),
        (status = 404, description = "NOT FOUND", body = String),
        (status = 500, description = "INTERNAL SERVER ERROR", body = String)
    ),
    tag = "Scheduler"
)]
#[get("/scheduler")]
async fn scheduler() -> impl Responder {
    actix_rt::spawn(async {
        let mut interval = time::interval(Duration::from_secs(20));
        loop {
            interval.tick().await;
            println!("20 seconds");
        }
    });

    HttpResponse::Ok().body("Hello world!")
}