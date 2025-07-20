use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    JwtError(JwtError),
    ValidationError(String),
    NotFound(String),
    Unauthorized(String),
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: i32,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(e) => write!(f, "Database error: {}", e),
            AppError::JwtError(e) => write!(f, "Token error: {}", e),
            AppError::ValidationError(m)
            | AppError::NotFound(m)
            | AppError::Unauthorized(m)
            | AppError::InternalError(m) => write!(f, "{}", m),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DbError(_) | AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) | AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        let code = status.as_u16() as i32;
        HttpResponse::build(status).json(ErrorResponse { code, message })
    }
}

// 自动从 sea_orm::DbErr & jsonwebtoken::Error 转换
impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::DbError(e)
    }
}
impl From<JwtError> for AppError {
    fn from(e: JwtError) -> Self {
        AppError::JwtError(e)
    }
}
