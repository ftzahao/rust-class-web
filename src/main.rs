use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::{Compat, Compress, DefaultHeaders, NormalizePath, from_fn},
    web::Data,
};
use config::Config;
use state::{AppState, CARGO_PKG_NAME, CARGO_PKG_VERSION};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务启动中...");
    let config = Config::new();
    let _ = Config::tracing_init(&config).await;
    println!("日志记录器初始化完成");
    let db = Config::init_db(&config).await;

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(Compat::new(TracingLogger::default()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                    .supports_credentials(),
            )
            .wrap(from_fn(middleware::auth::auth))
            .wrap(Compress::default())
            .wrap(DefaultHeaders::new().add(("X-Version", CARGO_PKG_VERSION)))
            .app_data(Data::new(AppState { db: db.clone() }))
            .configure(handlers::config)
    });
    let server_host = config.server.host;
    let server_port = config.server.port;
    let server_bind =
        match config.server.enabled_tls.as_str() {
            "rustls-0_23" => http_server
                .bind_rustls_0_23((server_host, server_port), config.server.rustls_config()),
            "openssl" => http_server
                .bind_openssl((server_host, server_port), config.server.openssl_builder()),
            _ => http_server.bind((server_host, server_port)),
        };
    println!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} 服务启动成功！");
    config.server.print_server_startup_address();
    server_bind?.run().await
}
