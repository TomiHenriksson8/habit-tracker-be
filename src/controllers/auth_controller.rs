use crate::models::user::User;
use crate::repositories::user_repository::{get_user_by_email, save_user};
use crate::utils::{generate_jwt, hash_password, verify_password};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

pub async fn register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, Response> {
    // Check if the user already exists
    if get_user_by_email(&db, &payload.email)
        .await
        .unwrap_or(None)
        .is_some()
    {
        return Err((StatusCode::CONFLICT, "User already exists").into_response());
    }

    let hashed_password = hash_password(&payload.password);

    let new_user = User {
        username: payload.username.clone(),
        email: payload.email.clone(),
        hashed_password,
    };

    match save_user(&db, new_user).await {
        Ok(_) => Ok(Json(RegisterResponse {
            message: "User registered successfully".to_string(),
        })),
        Err(err) => {
            eprintln!("Failed to save user: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to save user").into_response())
        }
    }
}

pub async fn login(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Response> {
    match get_user_by_email(&db, &payload.email).await {
        Ok(Some(user)) => {
            // Verify the password
            if verify_password(&payload.password, &user.hashed_password) {
                let token = generate_jwt(&user.username);
                Ok(Json(LoginResponse { token }))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid password").into_response())
            }
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found").into_response()),
        Err(err) => {
            eprintln!("Failed to retrieve user: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve user").into_response())
        }
    }
}
