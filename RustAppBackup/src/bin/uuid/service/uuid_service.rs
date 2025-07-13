use std::io::Error;

use actix_web::{web::{Data, Json}, HttpRequest};
use uuid::Uuid;

use crate::{
    repository::uuid_repo::UUIDRepo, 
    models::user_model::User
};

pub fn get_uuid(db: Data<UUIDRepo>) -> Result<String, Error> {
    println!("get_uuid");
    let uuid = Uuid::new_v4().to_string();
    let data = User {
        id: "1".to_string(),
        name: "".to_string(),
        location: "".to_string(),
        title: "".to_string(),
    };
    println!("db.create_user");
    let _user_detail = db.create_user(data);
    // println!("{}", user_detail.await);
    // let _origin = req.headers().get("origin").unwrap().to_str();
    
    Ok(uuid)
}

pub async fn _get_new_duuid(db: Data<UUIDRepo>, new_user: Json<User>, req: HttpRequest) -> Result<String, Error> {
    let uuid = Uuid::new_v4().to_string();
    
    let data = User {
        id: "1".to_string(),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let _user_detail = db.create_user(data).await;
    let _origin = req.headers().get("origin").unwrap().to_str();
    
    Ok(uuid)
}