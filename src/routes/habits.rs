use crate::controllers::habit_controller::{
    create_habit, list_habits, HabitRequest, HabitResponse,
};
use axum::{
    routing::{get, post},
    Json, Router,
};

async fn create_habit_handler(Json(payload): Json<HabitRequest>) -> Json<HabitResponse> {
    Json(create_habit(payload).await)
}

async fn list_habits_handler() -> Json<Vec<HabitRequest>> {
    Json(list_habits().await)
}

pub fn habit_routes() -> Router {
    Router::new().route(
        "/habits",
        post(create_habit_handler).get(list_habits_handler),
    )
}
