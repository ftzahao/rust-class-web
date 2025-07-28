use crate::entity::{devices, users};
use crate::errors::AppError;
use crate::models::token::generate_token;
use crate::state::AppState;
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
struct LoginReq {
    #[validate(email)]
    email: String,
    pass_word: String,
}
#[derive(Deserialize, Serialize)]
struct LoginResp<T> {
    code: i32,
    data: T,
    message: &'static str,
}

#[post("/users/login")]
pub async fn login(
    data: web::Json<LoginReq>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_opt = users::Entity::find()
        .filter(users::Column::Email.eq(&data.email))
        .one(&app_data.db_pool)
        .await?;

    if let Some(user) = user_opt {
        if argon2::verify_encoded(&user.pass_word, data.pass_word.as_bytes())
            .map_err(|e| AppError::InternalError(format!("Password verification error: {}", e)))?
        {
            let token = generate_token(&user.email);
            let device = devices::ActiveModel {
                id: NotSet,
                user_id: Set(user.id),
                token: Set(token.clone()),
                ..Default::default()
            };
            if let Err(e) = device.insert(&app_data.db_pool).await {
                return Err(AppError::InternalError(format!("设备写入失败: {}", e)));
            }
            return Ok(HttpResponse::Ok().json(LoginResp {
                code: 200,
                data: json!({ "user": user, "token": token }),
                message: "Login successful",
            }));
        } else {
            return Ok(HttpResponse::Unauthorized().json(LoginResp::<()> {
                code: 401,
                data: (),
                message: "Invalid password",
            }));
        }
    }

    Ok(HttpResponse::Unauthorized().json(LoginResp::<()> {
        code: 401,
        data: (),
        message: "User not found",
    }))
}
