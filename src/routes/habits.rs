use crate::controllers::habit_controller::{
    create_habit, get_habit, list_habits, HabitRequest, HabitResponse,
};
use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};

async fn create_habit_handler(Json(payload): Json<HabitRequest>) -> Json<HabitResponse> {
    Json(create_habit(payload).await)
}

async fn list_habits_handler() -> Json<Vec<HabitRequest>> {
    Json(list_habits().await)
}

async fn get_habit_handler(habit_id: &str) -> Json<HabitRequest> {
    Json(get_habit(habit_id).await)
}

pub fn habit_routes() -> Router {
    Router::new().route(
        "/habits",
        get(list_habits_handler).post(create_habit_handler),
    )
}
