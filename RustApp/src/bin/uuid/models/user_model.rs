use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    pub name: String,
    pub location: String,
    pub title: String,
}
