#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}
