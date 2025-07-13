use actix_web::web::ServiceConfig;

#[path = "./auth/auth.rs"]
pub mod auth;
#[path = "../../../pkg/dotenv/dotenv.rs"] 
pub mod dotenv;
#[path = "../../../pkg/logger/logger.rs"] 
pub mod logger;
#[path = "../../middleware/middleware.rs"] 
pub mod middleware;
#[path = "./project-management/project_management.rs"]
pub mod project_management;
#[path = "../../repositories/repository.rs"] 
pub mod default_repository;
#[path = "../../routes/route.rs"] 
pub mod default_route;
#[path = "./web/index.rs"] 
pub mod web;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        auth::configure(config);
        default_route::configure(config);
        web::configure(config);
        project_management::configure(config);
    }
}