mod error;
mod handlers;
mod models;

use std::{env, net::SocketAddr, time::Duration};

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::{
    create_account, deposit, health, list_accounts, list_transactions, transfer, withdraw,
};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init_tracing();

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is required")?;

    let app_port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .map_err(|_| "APP_PORT must be a valid u16")?;

    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let db = create_pool(&database_url).await?;
    run_migrations(&db).await?;

    let state = AppState { db };
    let cors = build_cors_layer(frontend_origin);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/accounts", get(list_accounts).post(create_account))
        .route("/api/accounts/{id}/deposit", post(deposit))
        .route("/api/accounts/{id}/withdraw", post(withdraw))
        .route("/api/accounts/{id}/transactions", get(list_transactions))
        .route("/api/transfers", post(transfer))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], app_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("backend listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await
}

async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

fn build_cors_layer(frontend_origin: Option<String>) -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT]);

    match frontend_origin {
        Some(origin) if origin != "*" => match HeaderValue::from_str(&origin) {
            Ok(header) => base.allow_origin(header),
            Err(_) => {
                warn!(
                    %origin,
                    "Invalid FRONTEND_ORIGIN value. Falling back to permissive CORS."
                );
                base.allow_origin(Any)
            }
        },
        _ => base.allow_origin(Any),
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
