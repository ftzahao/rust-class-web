use crate::entity::users;
use crate::errors::AppError;
use crate::state::{ARGON2_SALT, AppState};
use actix_web::{HttpResponse, Result, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct PostReqJson<T> {
    code: i32,
    data: T,
    message: &'static str,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
struct CreateUser {
    name: String,
    #[validate(email(message = "无效的邮箱地址"))]
    email: String,
    pass_word: String,
}
#[post("/users/create")]
pub async fn create_user(
    params: web::Json<CreateUser>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    println!("{:#?}", params);
    // 参数验证
    if let Err(e) = params.validate() {
        // 提取所有错误消息，只保留自定义内容
        let msg = e
            .field_errors()
            .values()
            .flat_map(|errs| errs.iter().filter_map(|err| err.message.as_ref()))
            .map(|msg| msg.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(AppError::ValidationError(msg));
    }

    let argon2_config = argon2::Config::default();
    let hashed_password =
        argon2::hash_encoded(params.pass_word.as_bytes(), ARGON2_SALT, &argon2_config)
            .map_err(|e| AppError::InternalError(format!("{e}")))?;

    // 使用 sea-orm 创建用户
    let user = users::ActiveModel {
        id: NotSet, // 自增ID，不需要设置
        name: Set(params.name.to_string()),
        email: Set(params.email.to_string()),
        pass_word: Set(hashed_password),
        status: Set("normal".to_string()),
        create_time: Set(Utc::now()),
        update_time: Set(Utc::now()),
    };

    let insert_result = user.insert(&app_data.db_pool).await?;
    println!("插入结果: {:?}", insert_result);

    Ok(HttpResponse::Ok().json(PostReqJson {
        code: 200,
        data: insert_result, // sea-orm 插入成功返回记录而不是影响行数，这里简单返回1
        message: "ok",
    }))
}
