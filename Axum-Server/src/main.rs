mod commands;
mod error;
mod http;
mod models;
mod queries;
mod state;

use sqlx::postgres::PgPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    // Ejecuta migraciones automáticamente al arrancar
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let app = http::routes::router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
