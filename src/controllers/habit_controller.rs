use crate::db::get_db;
use crate::models::habit::Habit;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize}; // Import the trait for `try_collect`

/// Structs for request and response
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

/// Create a habit
pub async fn create_habit(payload: HabitRequest) -> HabitResponse {
    let collection = get_habits_collection().await;

    let new_habit = Habit {
        id: None,
        title: payload.title,
        description: payload.description,
        frequency: payload.frequency,
    };

    match collection.insert_one(new_habit, None).await {
        Ok(_) => HabitResponse {
            message: "Habit created successfully".to_string(),
        },
        Err(_) => HabitResponse {
            message: "Failed to create habit".to_string(),
        },
    }
}

/// List all habits
pub async fn list_habits() -> Vec<Habit> {
    let collection = get_habits_collection().await;

    match collection.find(None, None).await {
        Ok(cursor) => {
            cursor.try_collect::<Vec<Habit>>().await.unwrap_or_default() // If an error occurs, return an empty vector
        }
        Err(_) => vec![], // Handle error by returning an empty list
    }
}

/// Get a habit by ID
pub async fn get_habit(habit_id: &str) -> Option<Habit> {
    let collection = get_habits_collection().await;

    match ObjectId::parse_str(habit_id) {
        Ok(object_id) => collection
            .find_one(doc! { "_id": object_id }, None)
            .await
            .ok()
            .flatten(),
        Err(_) => None,
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

            match collection
                .update_one(doc! { "_id": object_id }, update, None)
                .await
            {
                Ok(result) if result.modified_count > 0 => HabitResponse {
                    message: "Habit updated successfully".to_string(),
                },
                _ => HabitResponse {
                    message: "Habit not found or update failed".to_string(),
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
