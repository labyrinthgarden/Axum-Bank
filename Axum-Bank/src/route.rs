// src/app.rs
use tower_http::cors::{CorsLayer, Any};
use axum::http::HeaderValue;
use axum::Router;

pub fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/auth", auth::router())
        // ... más rutas
        .layer(cors)  // IMPORTANTE: aplicar DESPUÉS de las rutas
}
