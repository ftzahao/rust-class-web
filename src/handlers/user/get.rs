use crate::entity::users;
use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Info {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct PostReqJson<T> {
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
