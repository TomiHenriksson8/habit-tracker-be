use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct HabitRequest {
    pub title: String,
    pub description: Option<String>,
    pub frequency: String,
}

#[derive(Serialize)]
pub struct HabitResponse {
    pub message: String,
}

lazy_static! {
    static ref HABITS: Mutex<HashMap<String, HabitRequest>> = Mutex::new(HashMap::new());
}

pub async fn create_habit(payload: HabitRequest) -> HabitResponse {
    let habit_id = Uuid::new_v4().to_string();
    HABITS.lock().unwrap().insert(habit_id.clone(), payload);

    HabitResponse {
        message: "Habit created successfully".to_string(),
    }
}

pub async fn list_habits() -> Vec<HabitRequest> {
    let habits = HABITS.lock().unwrap();
    habits.values().cloned().collect()
}
