extern crate dotenv;

use bson::{doc, oid::ObjectId};

use crate::models::user_model::User;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client,
    Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = format!("{}&appName={}", dotenv!("MONGODB_URI"), dotenv!("APP_NAME"));
        let client = Client::with_uri_str(uri).await.unwrap().expect("Error getting client");
        let db = client.database(dotenv!("MONGODB_DATABASE"));
        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }

    pub async fn _create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: new_user.id,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        println!("Create User");
        println!("{:?}", user);
        Ok(user)
    }

    pub async fn _get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
}
