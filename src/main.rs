mod controllers;
mod db;
mod models;
mod repositories;
mod routes;
mod utils;

use axum::{extract::Extension, routing::get, Json, Router};
use db::get_database;
use mongodb::Database;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "Server is running".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let database = get_database().await;

    let shared_db = Arc::new(database);

    let app = Router::new()
        .route("/", get(health_check))
        .nest("/api/auth", routes::auth::auth_routes())
        .nest(
            "/api/habits",
            routes::habits::habit_routes().layer(Extension(shared_db.clone())),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to to start")
}
