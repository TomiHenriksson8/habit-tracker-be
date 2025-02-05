use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Habit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: Option<String>,
    pub frequency: String,
}
