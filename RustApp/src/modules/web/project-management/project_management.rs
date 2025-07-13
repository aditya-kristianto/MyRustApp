extern crate http;

use actix_web::Error;
use actix_web::get;
use actix_web::HttpRequest;
use actix_web::{HttpResponse, Result};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use actix_web::web::ServiceConfig;
use askama::Template;
use std::collections::HashMap;
use std::env;

#[path = "../../../repositories/duuid.rs"]
mod repository;

#[derive(Template)] // Define a struct that represents your template data.
#[template(path = "index.min.html")]
struct DashboardTemplate {
    asset_url: String,
}

pub(super) fn configure(config: &mut ServiceConfig) {
    config.service(home_page)
        .service(dashboard_page);
}

#[get("/")]
pub async fn home_page(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Found()
        .append_header(("Location", "/welcome"))
        .status(StatusCode::PERMANENT_REDIRECT)
        .finish())
}

#[get("/dashboard")]
pub async fn dashboard_page(req: HttpRequest) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
    let layout_param = params.get("layout");
    let _layout_value;
    match layout_param {
        Some(inner) => _layout_value = check_layout(inner.as_str()).unwrap(),
        None        => _layout_value = check_layout("").unwrap(),
    }

    let data = DashboardTemplate {
        asset_url: env::var("ASSET_URL").expect("ASSET_URL not found in the environment"),
    };

    let rendered = data.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

fn check_layout(layout: &str) -> Result<&str, Error> {
    let mut default_layout = "corporate";

    if layout == "corporate" || layout == "creative" || layout == "fancy" || layout == "overlay" {
        default_layout = layout;
    }

    Ok(default_layout)
}