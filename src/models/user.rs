
use mongodb::bson::oid::ObjectId; // Import ObjectId
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] // ✅ Ensure MongoDB `_id` handling
    pub id: Option<ObjectId>,  // ✅ Add user ID
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

