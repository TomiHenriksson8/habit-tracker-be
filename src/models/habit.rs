use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Habit {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub frequency: String,
    pub created_at: DateTime<Utc>,
}
