use actix_cors::Cors;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
        .supports_credentials()
}
