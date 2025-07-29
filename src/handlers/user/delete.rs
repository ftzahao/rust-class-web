use crate::entity::{devices, users};
use crate::errors::AppError;
use crate::state::AppState;
use crate::utils::extract_path_param;
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter};

#[derive(Deserialize, Serialize)]
pub struct PostReqJson<T> {
    code: i32,
    data: T,
    message: &'static str,
}

async fn find_user_active_model(
    user_id: i64,
    db_pool: &sea_orm::DatabaseConnection,
) -> Result<users::ActiveModel, AppError> {
    let model = users::Entity::find_by_id(user_id).one(db_pool).await?;
    if model.is_none() {
        return Err(AppError::NotFound(format!("用户ID {user_id} 不存在")));
    }
    Ok(model.unwrap().into())
}

async fn delete_user_devices(
    user_id: i64,
    db_pool: &sea_orm::DatabaseConnection,
) -> Result<DeleteResult, AppError> {
    let device_delete_result = devices::Entity::delete_many()
        .filter(crate::entity::devices::Column::UserId.eq(user_id))
        .exec(db_pool)
        .await?;
    println!("删除设备结果: {:?}", device_delete_result);
    Ok(device_delete_result)
}

#[delete("/users/delete/{id}")]
pub async fn delete_user(
    id: Result<web::Path<String>>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_path_param(id, "无效的用户ID")?.parse::<i64>()?;
    // 使用通用函数删除该用户下的所有设备
    delete_user_devices(user_id, &app_data.db_pool).await?;

    // 使用通用函数查找用户 ActiveModel
    let user = find_user_active_model(user_id, &app_data.db_pool).await?;

    let delete_result = user.delete(&app_data.db_pool).await?;
    println!("删除结果: {:?}", delete_result);
    Ok(HttpResponse::Ok().json(PostReqJson {
        code: 200,
        data: delete_result.rows_affected > 0, // sea-orm 删除成功返回影响行数
        message: "ok",
    }))
}
