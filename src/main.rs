mod controllers;
mod db;
mod models;
mod repositories;
mod routes;
mod utils;

use axum::{extract::Extension, routing::get, Json, Router};
use db::get_db;
use dotenvy::dotenv;
use serde::Serialize;
use std::env;
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
    // Load .env file
    dotenv().ok();

    // Debug print to ensure the environment variable is loaded
    if let Ok(uri) = env::var("MONGO_URI") {
        println!("MONGO_URI is set: {}", uri);
    } else {
        println!("MONGO_URI is not set!");
    }

    // Get the MongoDB database reference
    let database = get_db().await;

    // Share the database reference across routes
    let shared_db = Arc::new(database);

    // Set up the application router
    let app = Router::new()
        .route("/", get(health_check))
        .nest(
            "/api/auth",
            routes::auth::auth_routes().layer(Extension(shared_db.clone())), // Attach DB to auth routes
        )
        .nest(
            "/api/habits",
            routes::habits::habit_routes().layer(Extension(shared_db.clone())), // Attach DB to habits routes
        )
        .layer(TraceLayer::new_for_http());

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
