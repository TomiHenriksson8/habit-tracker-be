use axum::{routing::post, Json, Router};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
struct HabitRequest {
    title: String,
    description: Option<String>,
    frequency: String,
}

#[derive(Serialize)]
struct HabitResponse {
    message: String,
}

lazy_static! {
    static ref HABITS: Mutex<HashMap<String, HabitRequest>> = Mutex::new(HashMap::new());
}
async fn create_habit(Json(payload): Json<HabitRequest>) -> Json<HabitResponse> {
    let habit_id = Uuid::new_v4().to_string();
    HABITS.lock().unwrap().insert(habit_id.clone(), payload);

    Json(HabitResponse {
        message: "Habit created successfully".to_string(),
    })
}

async fn list_habits() -> Json<Vec<HabitRequest>> {
    let habits = HABITS.lock().unwrap();
    Json(habits.values().cloned().collect())
}

pub fn habit_routes() -> Router {
    Router::new().route("/habits", post(create_habit).get(list_habits))
}
