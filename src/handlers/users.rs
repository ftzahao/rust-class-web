use crate::entity::{devices, users};
use crate::errors::AppError;
use crate::models::token::generate_token;
use crate::state::{ARGON2_SALT, AppState};
use actix_web::{HttpResponse, Result, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DeleteResult, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
struct Info {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct PostReqJson<T> {
    code: i32,
    data: T,
    message: &'static str,
}

#[post("/users/getQueryUsers")]
pub async fn get_query_users(
    info: web::Json<Info>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    println!("{:#?}", info.name);

    // 使用 sea-orm 进行查询
    let user_list = users::Entity::find()
        .filter(crate::entity::users::Column::Name.contains(&info.name))
        .all(&app_data.db_pool)
        .await?;

    println!("{:#?}", user_list);
    Ok(HttpResponse::Ok().json(PostReqJson {
        code: 200,
        data: user_list,
        message: "ok",
    }))
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
        create_time: Set(Utc::now().naive_utc()),
        update_time: Set(Utc::now().naive_utc()),
    };

    let insert_result = user.insert(&app_data.db_pool).await?;
    println!("插入结果: {:?}", insert_result);

    Ok(HttpResponse::Ok().json(PostReqJson {
        code: 200,
        data: insert_result, // sea-orm 插入成功返回记录而不是影响行数，这里简单返回1
        message: "ok",
    }))
}

#[delete("/users/delete/{id}")]
pub async fn delete_user(
    id: web::Path<String>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let id_str = id.into_inner();
    let user_id: i64 = id_str
        .parse()
        .map_err(|_| AppError::ParseError(format!("无法解析用户ID: {id_str}")))?;
    println!("删除用户 ID: {}", user_id);

    // 先删除该用户下的所有设备
    let device_delete_result: DeleteResult = devices::Entity::delete_many()
        .filter(crate::entity::devices::Column::UserId.eq(user_id))
        .exec(&app_data.db_pool)
        .await?;
    println!("删除设备结果: {:?}", device_delete_result);

    // 使用 sea-orm 删除用户
    let model = users::Entity::find_by_id(user_id)
        .one(&app_data.db_pool)
        .await?;
    if model.is_none() {
        // 统一返回 AppError::NotFound，保证响应格式一致
        return Err(AppError::NotFound(format!("用户ID {user_id} 不存在")));
    }
    let user: users::ActiveModel = model.unwrap().into();

    let delete_result = user.delete(&app_data.db_pool).await?;
    println!("删除结果: {:?}", delete_result);
    Ok(HttpResponse::Ok().json(PostReqJson {
        code: 200,
        data: delete_result.rows_affected > 0, // sea-orm 删除成功返回影响行数
        message: "ok",
    }))
}

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

/// 用户登出请求的结构体
#[derive(Deserialize, Serialize)]
struct LogoutReq {
    id: i64,
    token: Option<String>,
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
