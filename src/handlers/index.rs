use crate::errors::AppError;
use actix_web::{Result, get};

#[get("/")]
pub async fn index() -> Result<String, AppError> {
    Ok("Hello, world!".into())
}
