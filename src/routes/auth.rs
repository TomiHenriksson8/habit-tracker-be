use crate::db::{get_user_by_email, save_user};
use crate::models::user::User;
use crate::utils::{generate_jwt, hash_password, verify_password};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    message: String,
}

async fn register(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    let user_id = Uuid::new_v4().to_string();

    // Hash the password using the utility function
    let hashed_password = hash_password(&payload.password);

    // Create and save the user
    let new_user = User {
        id: user_id,
        username: payload.username,
        email: payload.email,
        hashed_password,
    };

    save_user(new_user);

    Json(RegisterResponse {
        message: "User registered successfully".to_string(),
    })
}

async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if let Some(user) = get_user_by_email(&payload.email) {
        // Use verify_password to securely check the password
        if verify_password(&payload.password, &user.hashed_password) {
            let token = generate_jwt(&user.id);
            return Json(LoginResponse { token });
        }
    }

    Json(LoginResponse {
        token: "Invalid credentials".to_string(),
    })
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
