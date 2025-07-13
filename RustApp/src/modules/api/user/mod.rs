use actix_web::web::ServiceConfig;
pub mod repository;
pub mod v1;

#[path = "../../../repositories/repository.rs"] 
pub mod default_repository;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        v1::configure_v1(config);
    }
}