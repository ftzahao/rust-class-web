use crate::entity::{devices, users};
use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter};

#[derive(Deserialize, Serialize)]
pub struct PostReqJson<T> {
    code: i32,
    data: T,
    message: &'static str,
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
