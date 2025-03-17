
mod controllers;
mod db;
mod models;
mod repositories;
mod routes;
mod utils;

use axum::{extract::Extension, http::{Method, HeaderValue}, routing::get, Json, Router};
use db::get_db;
use dotenvy::dotenv;
use serde::Serialize;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any}; // Import CORS

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
    dotenv().ok();

    if let Ok(uri) = env::var("MONGO_URI") {
        println!("✅ MONGO_URI is set: {}", uri);
    } else {
        println!("❌ MONGO_URI is not set!");
    }

    let database = get_db().await;
    let shared_db = Arc::new(database);

    let frontend_url = "https://habit-tracker-dgcqa5p6t-tomi-s-projects.vercel.app";

    println!("🚀 Allowing CORS for frontend: {}", frontend_url);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .expose_headers(Any);

    let app = Router::new()
        .route("/", get(health_check))
        .nest(
            "/api/auth",
            routes::auth::auth_routes().layer(Extension(shared_db.clone())),
        )
        .nest(
            "/api/habits",
            routes::habits::habit_routes().layer(Extension(shared_db.clone())),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("❌ Failed to start server");
}
