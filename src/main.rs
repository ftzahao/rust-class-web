use rust_class_web::*;

use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    middleware,
    web::Data,
};
use state::{AppState, CARGO_PKG_NAME, CARGO_PKG_VERSION};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务启动中...");
    let app_config = app_config::Config::new();
    let app_state = AppState::new(&app_config)
        .await
        .map_err(std::io::Error::other)?;
    let _ = app_config.logger.tracing_init().await;

    let mut http_server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compat::new(TracingLogger::default()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
                    .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                    .supports_credentials(),
            )
            .wrap(middleware::from_fn(mw::auth))
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", CARGO_PKG_VERSION)))
            .app_data(Data::new(app_state.clone()))
            .configure(handlers::config)
    });
    let addr = app_config.server.addr();
    if app_config.server.enabled_tls {
        let tls = app_config.server.rustls_config();
        http_server = http_server.bind_rustls_0_23(addr, tls)?;
    } else {
        http_server = http_server.bind(addr)?;
    }
    println!("{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} 服务启动成功！");
    app_config.server.print_server_startup_address();
    http_server.run().await
}
