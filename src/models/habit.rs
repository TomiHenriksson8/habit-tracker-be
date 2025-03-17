
use mongodb::bson::{DateTime, oid::ObjectId}; 
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Habit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub title: String,
    pub description: Option<String>,
    pub frequency: String,     
    pub completed: bool,
    pub completion_count: u32,

    pub created_at: DateTime,  // ✅ MongoDB BSON DateTime  

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_completed: Option<DateTime>,

    #[serde(default)]
    pub completion_history: Vec<DateTime>,  

    pub user_id: String,
}

