use actix_web::web::ServiceConfig;
pub mod repository;
pub mod v1;

#[path = "../../../routes/route.rs"] 
pub mod default_route;

#[path = "../../../../pkg/dotenv/dotenv.rs"] 
pub mod dotenv;

#[path = "../../../repositories/mod.rs"] 
pub mod default_repository;

#[path = "../../../routes/api/route.rs"] 
pub mod route;

#[path = "../../../../pkg/swagger/swagger.rs"]
pub mod swagger;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        v1::configure_v1(config);
    }
}