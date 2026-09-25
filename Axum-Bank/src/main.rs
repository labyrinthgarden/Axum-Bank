mod app;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Servidor en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
