mod users;
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(users::get_query_users)
        .service(users::create_user)
        .service(users::delete_user);
    cfg.service(scope);
}
