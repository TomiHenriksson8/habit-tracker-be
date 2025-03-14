
use crate::models::user::User;
use crate::repositories::user_repository::{get_user_by_email, save_user};
use crate::utils::{generate_jwt, verify_password, hash_password, decode_jwt};
use axum::{
    extract::{Extension, Json, TypedHeader},
    headers::{Authorization, authorization::Bearer}, // ✅ Correct single import
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

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}

// 🟢 Register User
pub async fn register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, Response> {
    // Check if user already exists
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

// 🟢 User Login
pub async fn login(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Response> {
    match get_user_by_email(&db, &payload.email).await {
        Ok(Some(user)) => {
            // Verify password
            if verify_password(&payload.password, &user.hashed_password) {
                let token = generate_jwt(&user.email);
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

pub async fn get_user(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>, // ✅ Correct generic type
    Extension(db): Extension<Arc<Database>>,
) -> Result<Json<UserResponse>, Response> {
    let token = auth.token().to_string(); // ✅ Extracts the token correctly

    match decode_jwt(&token) {
        Ok(email) => {
            if let Some(user) = get_user_by_email(&db, &email).await.unwrap_or(None) {
                Ok(Json(UserResponse {
                    username: user.username,
                    email: user.email,
                }))
            } else {
                Err((StatusCode::NOT_FOUND, "User not found").into_response())
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token").into_response()),
    }
}
