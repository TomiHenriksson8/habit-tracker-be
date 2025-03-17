
use axum::{
    extract::{Json, TypedHeader, Path},
    headers::{Authorization, authorization::Bearer},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime},
    Collection,
};
use serde::{Deserialize, Serialize};
use crate::utils::decode_jwt;
use crate::db::get_db;
use crate::models::habit::Habit;
use crate::repositories::user_repository::get_user_by_email;
use chrono::{Utc, Local, DateTime};

/// Request structure for creating a habit
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

/// Function to get the MongoDB habits collection
pub async fn get_habits_collection() -> Collection<Habit> {
    let db = get_db().await;
    db.collection::<Habit>("habits")
}

/// ✅ **Create a new habit for the authenticated user**
pub async fn create_habit(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>, 
    Json(payload): Json<HabitRequest>,
) -> Result<Json<HabitResponse>, Response> {
    let collection = get_habits_collection().await;

    let user_email = match decode_jwt(auth.token()) {
        Ok(email) => email,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid token").into_response()),
    };

    let db = get_db().await;
    let user = match get_user_by_email(&db, &user_email).await {
        Ok(Some(user)) => user,
        _ => return Err((StatusCode::UNAUTHORIZED, "User not found").into_response()),
    };

    let new_habit = Habit {
    id: None,
    title: payload.title,
    description: payload.description,
    frequency: payload.frequency,
    completed: false,
    completion_count: 0,
    created_at: Utc::now().into(), // ✅ Fix here
    last_completed: None,
    user_id: user.id.unwrap().to_hex(),
    completion_history: vec![],
};    
match collection.insert_one(new_habit, None).await {
        Ok(_) => Ok(Json(HabitResponse {
            message: "Habit created successfully".to_string(),
        })),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create habit").into_response()),
    }
}
pub async fn complete_habit(
    Path(habit_id): Path<String>,
) -> Result<Json<HabitResponse>, Response> {
    let collection = get_habits_collection().await;

    let object_id = match ObjectId::parse_str(&habit_id) {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid habit ID").into_response()),
    };

    // 🔹 Fetch habit first
    let habit = match collection.find_one(doc! { "_id": &object_id }, None).await {
        Ok(Some(h)) => h,
        _ => return Err((StatusCode::NOT_FOUND, "Habit not found").into_response()),
    };

    // ✅ Get current UTC time and convert to BSON
    let now_bson = BsonDateTime::from_chrono(Utc::now());
    println!("✅ UTC Time: {}", now_bson);

    // ✅ Prevent duplicate completion for the same day
    let today_utc_str = now_bson.to_string(); // Convert BSON DateTime to string

    if habit.completion_history.iter().any(|date| date.to_string().contains(&today_utc_str)) {
        println!("⚠️ Habit already completed today, skipping...");
        return Ok(Json(HabitResponse {
            message: "Habit already marked as completed today.".to_string(),
        }));
    }

    let mut update_doc = doc! {
        "$push": { "completion_history": now_bson }, // ✅ Store UTC timestamp
    };

    let new_count = habit.completion_count + 1;
    let mut is_completed = false; // ✅ This variable was unused before, now used correctly.

    match habit.frequency.as_str() {
        "daily" => {
            is_completed = true;
            update_doc.insert("$set", doc! {
                "completed": true,
                "completion_count": 1,  // ✅ Reset daily count
                "last_completed": now_bson,  // ✅ Store last completed in UTC
            });
        }
        "weekly" => {
            if new_count >= 7 {
                is_completed = true;
            }
            update_doc.insert("$set", doc! {
                "completion_count": new_count,
                "completed": is_completed,
                "last_completed": now_bson,
            });
        }
        "monthly" => {
            if new_count >= 30 {
                is_completed = true;
            }
            update_doc.insert("$set", doc! {
                "completion_count": new_count,
                "completed": is_completed,
                "last_completed": now_bson,
            });
        }
        _ => {}
    };

    let update_result = collection.update_one(
        doc! { "_id": object_id },
        update_doc,
        None,
    ).await;

    match update_result {
        Ok(result) if result.modified_count > 0 => {
            println!("✅ Habit progress updated!");
            Ok(Json(HabitResponse {
                message: format!("Habit progress updated! {} / {}", habit.completion_count + 1, habit.frequency),
            }))
        }
        _ => Err((StatusCode::NOT_FOUND, "Habit not found or update failed.").into_response()),
    }
}pub async fn list_habits_by_user(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Habit>>, Response> {
    let collection = get_habits_collection().await;

    // ✅ Decode JWT to get user email
    let user_email = match decode_jwt(auth.token()) {
        Ok(email) => email,
        Err(_) => {
            println!("❌ Invalid token: {:?}", auth.token());
            return Err((StatusCode::UNAUTHORIZED, "Invalid token").into_response());
        }
    };

    let db = get_db().await;

    // ✅ Find the user by email
    let user = match get_user_by_email(&db, &user_email).await {
        Ok(Some(user)) => user,
        _ => {
            println!("❌ User not found: {}", user_email);
            return Err((StatusCode::UNAUTHORIZED, "User not found").into_response());
        }
    };

    // ✅ Ensure user_id is correctly extracted as a string
    let user_id = user.id.as_ref().unwrap().to_hex();
    println!("✅ Fetching habits for user_id: {}", user_id); // Debugging

    
let filter = doc! { "user_id": &user_id };

match collection.find(filter, None).await {
    Ok(cursor) => {
        let habits: Vec<Habit> = cursor.try_collect().await.unwrap_or_default();
        println!("✅ Retrieved habits: {:?}", habits);

        if habits.is_empty() {
            println!("⚠️ No habits found for user_id: {}", user_id);
        }

        Ok(Json(habits))
    }
    Err(err) => {
        println!("❌ Error fetching habits: {:?}", err);
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch habits").into_response())
    }
}
}
pub async fn get_habit(
    habit_id: &str,
) -> Result<Json<Option<Habit>>, Response> {
    let collection = get_habits_collection().await;

    match ObjectId::parse_str(&habit_id) {
        Ok(object_id) => {
            match collection.find_one(doc! { "_id": object_id }, None).await {
                Ok(habit) => Ok(Json(habit)),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch habit").into_response()),
            }
        }
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid habit ID").into_response()),
    }
}

/// Update a habit by ID
pub async fn update_habit(habit_id: &str, payload: HabitRequest) -> HabitResponse {
    let collection = get_habits_collection().await;

    match ObjectId::parse_str(habit_id) {
        Ok(object_id) => {
            let update = doc! {
                "$set": {
                    "title": payload.title,
                    "description": payload.description,
                    "frequency": payload.frequency,
                }
            };

            match collection.update_one(doc! { "_id": object_id }, update, None).await {
                Ok(result) if result.modified_count > 0 => HabitResponse {
                    message: "Habit updated successfully".to_string(),
                },
                _ => HabitResponse {
                    message: "Habit not found or update failed.".to_string(),
                },
            }
        }
        Err(_) => HabitResponse {
            message: "Invalid habit ID".to_string(),
        },
    }
}


/// Delete a habit by ID
pub async fn delete_habit(habit_id: &str) -> HabitResponse {
    let collection = get_habits_collection().await;

    match ObjectId::parse_str(habit_id) {
        Ok(object_id) => match collection.delete_one(doc! { "_id": object_id }, None).await {
            Ok(result) if result.deleted_count > 0 => HabitResponse {
                message: "Habit deleted successfully".to_string(),
            },
            _ => HabitResponse {
                message: "Habit not found".to_string(),
            },
        },
        Err(_) => HabitResponse {
            message: "Invalid habit ID".to_string(),
        },
    }
}
