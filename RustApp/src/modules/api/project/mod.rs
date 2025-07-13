use actix_web::web::ServiceConfig;
pub mod v1;

#[path = "../../../repositories/repository.rs"] 
pub mod default_repository;
#[path = "../../../routes/route.rs"] 
pub mod default_route;
#[path = "../../../../pkg/dotenv/dotenv.rs"] 
pub mod dotenv;
#[path = "../../../../pkg/logger/logger.rs"] 
pub mod logger;
#[path = "../../../middleware/middleware.rs"] 
pub mod middleware;
#[path = "../../../../pkg/db/postgres/postgres.rs"] 
pub mod postgres;
pub mod repository;
#[path = "../../../routes/api/route.rs"] 
pub mod route;
#[path = "../../../../pkg/swagger/swagger.rs"]
pub mod swagger;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        v1::configure_v1(config);
    }
}