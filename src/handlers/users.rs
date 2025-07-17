use crate::entity::users::{ActiveModel as UserActiveModel, Entity as Users, Model as UserModel};
use crate::state::AppState;
use actix_web::{
    Responder, Result, delete, post,
    web::{Data, Json, Path},
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

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
pub async fn get_query_users(info: Json<Info>, app_data: Data<AppState>) -> Result<impl Responder> {
    println!("{:#?}", info.name);

    // 使用 sea-orm 进行查询
    let user_list = Users::find()
        .filter(crate::entity::users::Column::Name.contains(&info.name))
        .all(&app_data.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("{:#?}", user_list);
    let data = PostReqJson {
        code: 200,
        data: user_list,
        message: "ok",
    };
    Ok(Json(data))
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateUser {
    name: String,
    email: String,
    pass_word: String,
}
#[post("/users/create")]
pub async fn create_user(
    params: Json<CreateUser>,
    app_data: Data<AppState>,
) -> Result<impl Responder> {
    println!("{:#?}", params);

    // 使用 sea-orm 创建用户
    let user = UserActiveModel {
        id: NotSet, // 自增ID，不需要设置
        name: Set(params.name.to_string()),
        email: Set(params.email.to_string()),
        pass_word: Set(params.pass_word.to_string()),
        status: Set("normal".to_string()),
        create_time: Set(Utc::now().naive_utc()),
        update_time: Set(Utc::now().naive_utc()),
    };

    let insert_result = user.insert(&app_data.db).await.unwrap();
    println!("插入结果: {:?}", insert_result);

    let data = PostReqJson {
        code: 200,
        data: insert_result, // sea-orm 插入成功返回记录而不是影响行数，这里简单返回1
        message: "ok",
    };
    Ok(Json(data))
}

#[delete("/users/delete/{id}")]
pub async fn delete_user(id: Path<i32>, app_data: Data<AppState>) -> Result<impl Responder> {
    let user_id = id.into_inner();
    println!("删除用户 ID: {}", user_id);
    // 使用 sea-orm 删除用户
    let user: UserActiveModel = Users::find_by_id(user_id)
        .one(&app_data.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?
        .into();

    let delete_result = user
        .delete(&app_data.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    println!("删除结果: {:?}", delete_result);
    let data = PostReqJson {
        code: 200,
        data: delete_result.rows_affected > 0, // sea-orm 删除成功返回影响行数
        message: "ok",
    };
    Ok(Json(data))
}
