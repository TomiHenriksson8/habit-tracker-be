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
pub struct HabitWithId {
    pub habit_id: String,
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

pub async fn get_habit(habit_id: &str) -> Option<HabitRequest> {
    let habits = HABITS.lock().unwrap();
    habits.get(habit_id).cloned()
}

pub async fn update_habit(habit_id: &str, payload: HabitWithId) -> HabitResponse {
    let mut habits = HABITS.lock().unwrap();
    if habits.contains_key(habit_id) {
        habits.insert(habit_id.to_string(), payload);
        HabitResponse {
            message: "Habit updated successfully".to_string(),
        }
    } else {
        HabitResponse {
            message: "Habit nor found".to_string(),
        }
    }
}

pub async fn delete_habit(habit_id: &str) -> HabitResponse {
    let mut habits = HABITS.lock().unwrap();
    if habits.remove(habit_id).is_some() {
        HabitResponse {
            message: "Habit deleted successfully".to_string(),
        }
    } else {
        HabitResponse {
            message: "Habit not found".to_string(),
        }
    }
}
