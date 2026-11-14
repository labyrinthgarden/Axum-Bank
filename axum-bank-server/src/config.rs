pub struct Config {
  pub database_url: String,
  pub server_addr: String
}

impl Config {
  pub fn from_env() -> anyhow::Result<Self> {
    Ok(Self {
      database_url: std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?,
      server_addr: std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
    })
  }
}
