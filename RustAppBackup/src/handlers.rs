use actix_web::Responder;
use actix_web::get;
use actix_web::post;
use actix_web::delete;
use actix_web::web::ServiceConfig;

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(get_users)
            .service(get_user_by_id)
            .service(add_user)
            .service(delete_user);
    }
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

#[post("/users")]
pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

#[delete("/users")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}