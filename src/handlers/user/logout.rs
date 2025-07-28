use crate::entity::devices;
use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

/// 用户登出请求的结构体
#[derive(Deserialize, Serialize)]
struct LogoutReq {
    id: i64,
    token: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct LoginResp<T> {
    code: i32,
    data: T,
    message: &'static str,
}

/// 处理用户登出请求
#[post("/logout")]
pub async fn logout(
    data: web::Json<LogoutReq>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let mut query = devices::Entity::delete_many().filter(devices::Column::UserId.eq(data.id));
    // token 可传可不传，传空字符串也视为删除所有
    if let Some(token) = &data.token {
        if !token.is_empty() {
            query = query.filter(devices::Column::Token.eq(token));
        }
    }
    let _ = query.exec(&app_data.db_pool).await;

    Ok(HttpResponse::Ok().json(LoginResp {
        code: 200,
        data: true,
        message: "Logout successful",
    }))
}
