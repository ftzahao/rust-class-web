use actix_web::{Result, get};

#[get("/")]
pub async fn index() -> Result<String> {
    Ok("Hello, world!".into())
}
