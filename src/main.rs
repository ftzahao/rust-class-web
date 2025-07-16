use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::{Compress, Logger},
    web::Data,
};
use config::Config;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let config = Config::new();

    let pool = db::init_db(&config).await;
    println!("服务器成功启动!");

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .configure(handlers::config)
    });
    let server_bind = match config.tls.enabled.as_str() {
        "rustls-0_23" => {
            print!("Using Rustls 0.23 for TLS");
            http_server.bind_rustls_0_23(
                (config.server.host, config.server.port),
                Config::rustls_config(&config),
            )
        }
        "openssl" => {
            print!("Using OpenSSL for TLS");
            http_server.bind_openssl(
                (config.server.host, config.server.port),
                Config::openssl_builder(&config),
            )
        }
        _ => http_server.bind((config.server.host, config.server.port)),
    };
    server_bind?.run().await
}
