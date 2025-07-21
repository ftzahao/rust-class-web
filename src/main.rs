use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware::{Compat, Compress, DefaultHeaders, NormalizePath, from_fn},
    web::Data,
};
use config::server::EnabledTls;
use state::{AppState, CARGO_PKG_NAME, CARGO_PKG_VERSION};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务启动中...");
    let app_state = AppState::new().await.expect("AppState 初始化失败");
    let _ = app_state.config.logger.tracing_init().await;
    let app_state_config_server = app_state.config.server.clone();

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
            .app_data(Data::new(app_state.clone()))
            .configure(handlers::config)
    });
    let server_bind = match app_state_config_server.enabled_tls {
        EnabledTls::Mode(ref s) if s == "rustls-0_23" => http_server.bind_rustls_0_23(
            app_state_config_server.addr(),
            app_state_config_server.rustls_config(),
        ),
        EnabledTls::Mode(ref s) if s == "openssl" => http_server.bind_openssl(
            app_state_config_server.addr(),
            app_state_config_server.openssl_builder(),
        ),
        EnabledTls::Enabled => http_server.bind_rustls_0_23(
            app_state_config_server.addr(),
            app_state_config_server.rustls_config(),
        ),
        _ => http_server.bind(app_state_config_server.addr()),
    };
    println!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} 服务启动成功！");
    app_state_config_server.print_server_startup_address();
    server_bind?.run().await
}
