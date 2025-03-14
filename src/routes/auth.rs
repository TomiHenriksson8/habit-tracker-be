use axum::{routing::{post, get}, Router};
use crate::controllers::auth_controller::{login, register, get_user};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/me", get(get_user))
}

