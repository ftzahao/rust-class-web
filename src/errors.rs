use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt;

/// 应用程序的统一错误类型
#[derive(Debug)]
pub enum AppError {
    /// 数据库相关错误
    DbError(DbErr),
    /// JWT 相关错误
    JwtError(JwtError),
    /// 校验失败错误，包含错误信息
    ValidationError(String),
    /// 资源未找到错误，包含错误信息
    NotFound(String),
    /// 未授权访问错误，包含错误信息
    Unauthorized(String),
    /// 内部服务器错误，包含错误信息
    InternalError(String),
    /// 解析错误，包含错误信息
    ParseError(String),
    /// 冲突错误，包含错误信息
    Conflict(String),
    /// 禁止访问错误，包含错误信息
    Forbidden(String),
    /// 请求超时错误，包含错误信息
    Timeout(String),
    /// 错误请求，包含错误信息
    BadRequest(String),
    /// 服务不可用，包含错误信息
    ServiceUnavailable(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: i32,
    message: String,
    data: (),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(e) => write!(f, "{e}"),
            AppError::JwtError(e) => write!(f, "{e}"),
            AppError::ValidationError(m)
            | AppError::NotFound(m)
            | AppError::Unauthorized(m)
            | AppError::InternalError(m)
            | AppError::ParseError(m)
            | AppError::Conflict(m)
            | AppError::Forbidden(m)
            | AppError::Timeout(m)
            | AppError::BadRequest(m)
            | AppError::ServiceUnavailable(m) => write!(f, "{m}"),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DbError(_) | AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) | AppError::ParseError(_) | AppError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) | AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        let code = status.as_u16() as i32;
        HttpResponse::build(status).json(ErrorResponse {
            code,
            message,
            data: (),
        })
    }
}

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

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::ParseError(format!("{e}"))
    }
}
