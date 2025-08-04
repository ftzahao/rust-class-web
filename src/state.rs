use anyhow::Result;
use mongodb::Client;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: sea_orm::DatabaseConnection,
    pub mongodb_client: Client,
}

impl AppState {
    pub async fn new(app_config: &crate::app_config::Config) -> Result<Self> {
        let db_pool = app_config.db.init_db().await;
        let mongodb_client = app_config.mongodb.client().await?;
        Ok(Self {
            db_pool,
            mongodb_client,
        })
    }
}

/// `Cargo.toml` 中的 package.name
pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
/// `Cargo.toml` 中的 package.version
pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// argon2 盐值
pub const ARGON2_SALT: &[u8] = b"81d84995-8531-49b2-b563-12b0e17bc784";
/// TOKEN 过期时间
pub const TOKEN_EXPIRE_TIME: i64 = 60 * 60 * 24 * 7; // 7 天
