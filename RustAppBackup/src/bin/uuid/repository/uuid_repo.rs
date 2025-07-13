extern crate dotenv;

use bson::{doc, oid::ObjectId};

use crate::models::user_model::User;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client,
    Collection,
};

pub struct UUIDRepo {
    col: Collection<User>,
}

impl UUIDRepo {
    pub async fn init() -> Self {
        println!("UUIDRepo init");
        let uri = format!("{}&appName={}", dotenv!("MONGODB_URI"), dotenv!("APP_NAME"));
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(dotenv!("MONGODB_DATABASE"));
        let col: Collection<User> = db.collection("uuids");
        
        // println!("{}", db);

        UUIDRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        println!("Create User 2");
        let new_doc = User {
            id: new_user.id,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc)
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
            .find_one(filter)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
}
