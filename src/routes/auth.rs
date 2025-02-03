use crate::db::get_user_by_email;
use crate::utils::generate_jwt;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    if let Some(user) = get_user_by_email(&payload.email) {
        if payload.password == user.hashed_password {
            let token = generate_jwt(&user.id);
            return Json(LoginResponse { token });
        }
    }

    Json(LoginResponse {
        token: "Invalid credintials".to_string(),
    })
}

pub fn auth_routes() -> Router {
    Router::new().route("/login", post(login))
}
