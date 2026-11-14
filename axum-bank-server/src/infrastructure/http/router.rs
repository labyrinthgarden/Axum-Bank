use axum::{routing::post, Router};

use crate::app_state::AppState;
use crate::infrastructure::http::handlers::account_handler::create_account;

pub fn build_router(state: AppState) -> Router {
  Router::new()
    .route("/accounts", post(create_account))
    .with_state(state)
}
