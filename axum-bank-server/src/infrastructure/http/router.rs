use axum::{routing::{get,post}, Router};

use crate::app_state::AppState;
use crate::infrastructure::http::handlers::account_handler::{create_account,get_account};

pub fn build_router(state: AppState) -> Router {
  Router::new()
    .route("/accounts", post(create_account))
    .route("/accounts/{id}",get(get_account))
    .with_state(state)
}
