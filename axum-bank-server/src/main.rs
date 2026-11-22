use std::sync::Arc;

use axum_bank_server::{
  domain::repositories::account_repository::AccountRepository,
  app_state::AppState,
  application::use_cases::create_account::CreateAccountUseCase,
  application::use_cases::get_account::GetAccountUseCase,
  application::use_cases::deposit::DepositUseCase,
  config::Config,
  infrastructure::{
    http::router::build_router,
    persistence::pg_account_repository::PgAccountRepository,
  },
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = Config::from_env()?;
  let pool = PgPool::connect(&config.database_url).await?;
  sqlx::migrate!("./migrations").run(&pool).await?;

  let repo: Arc<dyn AccountRepository> = Arc::new(PgAccountRepository::new(pool));

  let create_account_uc = Arc::new(CreateAccountUseCase::new(Arc::clone(&repo)));
  let get_account_uc = Arc::new(GetAccountUseCase::new(Arc::clone(&repo)));
  let deposit_uc = Arc::new(DepositUseCase::new(repo));

  let state = AppState {
    create_account: create_account_uc,
    get_account: get_account_uc,
    deposit: deposit_uc
  };

  let app = build_router(state);
  let listener = tokio::net::TcpListener::bind(&config.server_addr).await?;
  tracing::info!("listening on {}", config.server_addr);
  axum::serve(listener, app).await?;
  Ok(())
}
