





use crate::controllers::habit_controller::{
    create_habit, delete_habit, get_habit, list_habits_by_user, update_habit, complete_habit,
    HabitRequest, HabitResponse,
};
use crate::models::habit::Habit;
use axum::{
    extract::{Json, Path, TypedHeader},
    headers::{Authorization, authorization::Bearer},
    routing::{delete, get, post, put},
    Router,
    response::Response,
};

/// ✅ **Handler: Create a Habit**
async fn create_habit_handler(
    auth: TypedHeader<Authorization<Bearer>>, 
    Json(payload): Json<HabitRequest>,
) -> Result<Json<HabitResponse>, Response> {
    create_habit(auth, Json(payload)).await
}

/// ✅ **Handler: List Habits for Authenticated User**
async fn list_habits_by_user_handler(
    auth: TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Habit>>, Response> {
    list_habits_by_user(auth).await
}

/// ✅ **Handler: Get a Habit by ID**
async fn get_habit_handler(
    Path(habit_id): Path<String>,
) -> Result<Json<Option<Habit>>, Response> {
    get_habit(habit_id.as_str()).await  // ✅ Convert `String` to `&str`
}

/// ✅ **Handler: Update a Habit**
async fn update_habit_handler(
    Path(habit_id): Path<String>,
    Json(payload): Json<HabitRequest>,
) -> Result<Json<HabitResponse>, Response> {
    Ok(Json(update_habit(habit_id.as_str(), payload).await))  // ✅ Convert `String` to `&str`
}

/// ✅ **Handler: Delete a Habit**
async fn delete_habit_handler(
    Path(habit_id): Path<String>,
) -> Result<Json<HabitResponse>, Response> {
    Ok(Json(delete_habit(habit_id.as_str()).await))  // ✅ Convert `String` to `&str`
}

/// ✅ **Handler: Mark a Habit as Completed**
async fn complete_habit_handler(
    Path(habit_id): Path<String>,
) -> Result<Json<HabitResponse>, Response> {
    complete_habit(Path(habit_id)).await  // ✅ Pass `Path<String>` directly
}

/// 🔥 **Define Habit Routes**
pub fn habit_routes() -> Router {
    Router::new()
        .route("/", post(create_habit_handler))  // 🔹 Create a new habit
        .route("/", get(list_habits_by_user_handler))  // 🔹 Get habits for logged-in user
        .route("/:id", get(get_habit_handler))  // 🔹 Fetch habit by ID
        .route("/:id", put(update_habit_handler)) // 🔹 Update a habit
        .route("/:id/complete", put(complete_habit_handler)) // 🔹 Mark a habit as completed
        .route("/:id", delete(delete_habit_handler)) // 🔹 Delete a habit
}

