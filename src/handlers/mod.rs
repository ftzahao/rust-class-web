mod index;
use actix_web::web::{ServiceConfig, scope};
mod user;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index).service(
        scope("/api")
            .service(user::get::get_query_users)
            .service(user::delete::delete_user)
            .service(user::login::login)
            .service(user::logout::logout)
            .service(user::create::create_user),
    );
}
