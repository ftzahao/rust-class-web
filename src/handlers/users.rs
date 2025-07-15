use crate::state::AppState;
use actix_web::{
    Responder, Result, delete, post,
    web::{Data, Json, Path},
};
use sqlx::{FromRow, query, query_as};

#[derive(Deserialize, Serialize, Debug, FromRow)]
struct Info {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct PostReqJson<T> {
    code: i32,
    data: T,
    message: &'static str,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct UserModel {
    id: i32,
    name: String,
    email: String,
    pass_word: String,
    create_time: String,
    update_time: String,
    status: String,
}

#[post("/users/getQueryUsers")]
pub async fn get_query_users(info: Json<Info>, app_data: Data<AppState>) -> Result<impl Responder> {
    println!("{:#?}", info.name);
    let user_list: Vec<UserModel> =
        query_as::<_, UserModel>("SELECT * FROM users WHERE name LIKE ?")
            .bind(format!("{}%", info.name.to_string()))
            .fetch_all(&app_data.pool)
            .await
            .unwrap();
    println!("{:#?}", user_list);
    let data = PostReqJson {
        code: 200,
        data: user_list,
        message: "ok",
    };
    Ok(Json(data))
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct CreateUser {
    name: String,
    email: String,
    pass_word: String,
}
#[post("/users/create")]
pub async fn create_user(
    info: Json<CreateUser>,
    app_data: Data<AppState>,
) -> Result<impl Responder> {
    println!("{:#?}", info);
    let flag =
        query("INSERT INTO users (name, email, pass_word, status) VALUES (?, ?, ?, 'normal')")
            .bind(info.name.to_string())
            .bind(info.email.to_string())
            .bind(info.pass_word.to_string())
            .execute(&app_data.pool)
            .await
            .unwrap();

    let data = PostReqJson {
        code: 200,
        data: flag.rows_affected(),
        message: "ok",
    };
    Ok(Json(data))
}

#[delete("/users/delete/{id}")]
pub async fn delete_user(id: Path<i32>, app_data: Data<AppState>) -> Result<impl Responder> {
    let flag = query("DELETE FROM users WHERE id = ?")
        .bind(id.into_inner())
        .execute(&app_data.pool)
        .await
        .unwrap();

    let data = PostReqJson {
        code: 200,
        data: flag.rows_affected(),
        message: "ok",
    };
    Ok(Json(data))
}
