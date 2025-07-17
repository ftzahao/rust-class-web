#[macro_use]
extern crate tracing;

use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::{Compat, Compress, DefaultHeaders},
    web::Data,
};
use config::Config;
use state::{AppState, CARGO_PKG_VERSION};

use tracing_actix_web::TracingLogger;

mod entity;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务启动中...");
    let _guard = log::tracing_init();
    debug!("日志记录器已初始化");

    let config = Config::new();

    let db = Config::init_db(&config).await;

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Compat::new(TracingLogger::default()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                    .supports_credentials(),
            )
            .wrap(Compress::default())
            .wrap(DefaultHeaders::new().add(("X-Version", CARGO_PKG_VERSION)))
            .app_data(Data::new(AppState { db: db.clone() }))
            .configure(handlers::config)
    });
    let server_host = config.server.host;
    let server_port = config.server.port;
    let server_bind = match config.tls.enabled.as_str() {
        "rustls-0_23" => {
            debug!("使用SSL(Rustls)协议启动服务，监听地址: https://{server_host}:{server_port}");
            http_server.bind_rustls_0_23((server_host, server_port), Config::rustls_config(&config))
        }
        "openssl" => {
            debug!("使用SSL(OpenSSL)协议启动服务，监听地址: https://{server_host}:{server_port}");
            http_server.bind_openssl((server_host, server_port), Config::openssl_builder(&config))
        }
        _ => {
            debug!("使用 HTTP 协议启动服务，监听地址: http://{server_host}:{server_port}");
            http_server.bind((server_host, server_port))
        }
    };
    server_bind?.run().await
}
