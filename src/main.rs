#[macro_use]
extern crate tracing;

use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::{Compat, Compress},
    web::Data,
};
use config::Config;
use state::AppState;

use tracing_actix_web::TracingLogger;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server...");
    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(
        "[",
        env!("CARGO_PKG_NAME"),
        "].[",
        env!("CARGO_PKG_VERSION"),
        "].log"
    )
    .to_string();
    let rolling_file_appender =
        RollingFileAppender::new(Rotation::HOURLY, "./data/logs", app_name.clone());

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(rolling_file_appender);
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new(tracing::Level::DEBUG.to_string()))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let config = Config::new();

    let pool = db::init_db(&config).await;

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
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .configure(handlers::config)
    });
    let server_bind = match config.tls.enabled.as_str() {
        "rustls-0_23" => {
            info!(
                "使用TLS(Rustls 0.23)协议启动服务，监听地址: https://{}:{}",
                config.server.host, config.server.port
            );
            http_server.bind_rustls_0_23(
                (config.server.host, config.server.port),
                Config::rustls_config(&config),
            )
        }
        "openssl" => {
            tracing::info!(
                "使用TLS(OpenSSL)协议启动服务，监听地址: https://{}:{}",
                config.server.host,
                config.server.port
            );
            http_server.bind_openssl(
                (config.server.host, config.server.port),
                Config::openssl_builder(&config),
            )
        }
        _ => {
            info!(
                "使用 HTTP 协议启动服务，监听地址: http://{}:{}",
                config.server.host, config.server.port
            );
            http_server.bind((config.server.host, config.server.port))
        }
    };
    server_bind?.run().await
}
