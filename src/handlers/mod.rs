mod index;
mod users;
use actix_web::web::{ServiceConfig, scope};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index).service(
        scope("/api")
            .service(users::get_query_users)
            .service(users::delete_user)
            .service(users::login)
            .service(users::create_user),
    );
}
