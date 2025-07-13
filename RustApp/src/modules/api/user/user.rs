use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"username": "aditya.kristianto", "email": "kristianto.aditya@gmail.com", "password_hash": "mysecretpassword", "fullname": "Aditya Kristianto"}))]
pub struct UserPOSTRequest {
    username: String,
    email: String,
    password_hash: String,
    full_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPUTRequest {
    user_id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    full_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserGetRequest {
    user_id: Uuid,
    username: String,
    email: String,
    full_name: String,
}

impl User {
    #[allow(dead_code)]
    pub fn get_user_id(&self) -> &Uuid {
        &self.user_id
    }

    #[allow(dead_code)]
    pub fn get_username(&self) -> &String {
        &self.username
    }

    #[allow(dead_code)]
    pub fn get_email(&self) -> &String {
        &self.username
    }

    #[allow(dead_code)]
    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }

    #[allow(dead_code)]
    pub fn get_full_name(&self) -> &String {
        &self.full_name
    }
}

#[allow(dead_code)]
pub fn new(username: String, email: String, password_hash: String, full_name: String) -> User {
    User {
        user_id: Uuid::new_v4(),
        username: username,
        email: email,
        password_hash: password_hash,
        full_name: full_name,
        created_at: Utc::now(),
        created_by: Uuid::new_v4(),
        updated_at: None,
        updated_by: None,
        deleted_at: None,
        deleted_by: None,
    }
}

#[allow(dead_code)]
pub fn new_user(user_id: Uuid, username: String, email: String, full_name: String) -> UserGetRequest {
    UserGetRequest {
        user_id: user_id,
        username: username,
        email: email,
        full_name: full_name,
    }
}