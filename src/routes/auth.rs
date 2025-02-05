use crate::controllers::auth_controller::{login, register};
use axum::{routing::post, Router};

/// Auth routes with handlers for login and register
pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
