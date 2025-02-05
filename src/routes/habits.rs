use crate::controllers::habit_controller::{
    create_habit, delete_habit, get_habit, list_habits, update_habit, HabitRequest, HabitResponse,
};
use crate::models::habit::Habit;
use axum::{routing::delete, routing::get, routing::post, routing::put, Json, Router};

/// Handlers for routes
async fn create_habit_handler(Json(payload): Json<HabitRequest>) -> Json<HabitResponse> {
    Json(create_habit(payload).await)
}

async fn list_habits_handler() -> Json<Vec<Habit>> {
    Json(list_habits().await)
}

async fn get_habit_handler(
    axum::extract::Path(habit_id): axum::extract::Path<String>,
) -> Json<Option<Habit>> {
    Json(get_habit(&habit_id).await)
}

async fn update_habit_handler(
    axum::extract::Path(habit_id): axum::extract::Path<String>,
    Json(payload): Json<HabitRequest>,
) -> Json<HabitResponse> {
    Json(update_habit(&habit_id, payload).await)
}

async fn delete_habit_handler(
    axum::extract::Path(habit_id): axum::extract::Path<String>,
) -> Json<HabitResponse> {
    Json(delete_habit(&habit_id).await)
}

/// Define routes for habit operations
pub fn habit_routes() -> Router {
    Router::new()
        .route("/", get(list_habits_handler))
        .route("/", post(create_habit_handler))
        .route("/:id", get(get_habit_handler))
        .route("/:id", put(update_habit_handler))
        .route("/:id", delete(delete_habit_handler))
}
