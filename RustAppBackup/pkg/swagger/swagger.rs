extern crate openapi;

use std::fs::File;
use std::io::prelude::*;
use utoipa::OpenApi;
use actix_web::{get, HttpResponse, Responder};

#[path = "../dotenv/dotenv.rs"] mod dotenv;

#[derive(OpenApi)]
#[openapi(components())]
struct ApiDoc;

#[allow(dead_code)]
pub async fn init(openapi: &mut utoipa::openapi::OpenApi) -> std::io::Result<()> {
    // Get the version dynamically
    let dynamic_version = dotenv::get_app_version();

    // Explicitly update the version field
    openapi.info.version = dynamic_version;

    let mut file = File::create("docs/swagger.json")?;
    
    let swagger_json = openapi.to_pretty_json()?;
    file.write_all(swagger_json.as_bytes())?;

    Ok(())
}

#[get("/swagger/json")]
async fn get_swagger_json() -> impl Responder {
    let swagger_json =  ApiDoc::openapi().to_pretty_json().unwrap();

    HttpResponse::Ok().body(swagger_json.to_string())
} 