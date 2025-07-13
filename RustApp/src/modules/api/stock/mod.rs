use actix_web::web::ServiceConfig;
pub mod repository;
pub mod v1;
pub mod v2;

#[path = "../../../../pkg/dotenv/dotenv.rs"] 
pub mod dotenv;

#[path = "../../../../pkg/logger/logger.rs"] 
pub mod logger;

#[path = "../../../../pkg/swagger/swagger.rs"]
pub mod swagger;

#[path = "../../../../pkg/db/postgres/postgres.rs"] 
pub mod postgres;

#[path = "../../../../pkg/db/mongodb/mongodb.rs"] 
pub mod mongodb;

#[path = "../../../middleware/middleware.rs"] 
pub mod middleware;

#[path = "../../../routes/route.rs"] 
pub mod default_route;

#[path = "../../../routes/api/route.rs"] 
pub mod route;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        v1::configure_v1(config);
        v2::configure_v2(config);
    }
}