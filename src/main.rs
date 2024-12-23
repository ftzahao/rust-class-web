#[macro_use]
extern crate serde;

mod db;
mod env;
mod load_rustls_config;
mod log;
mod middleware;
mod models;
mod routes;
mod utils;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::load_dotenv().expect("main::env::load_dotenv()");
    env_logger::init();
    log::load_log().expect("main::log::load_log()");
    let pool = db::init_db().await;
    println!("服务器成功启动!");
    let service_port = std::env::var("SERVICE_PORT")
        .unwrap_or("8001".to_string())
        .parse::<u16>()
        .unwrap_or(8001);
    let enable_tls = std::env::var("ENABLE_TLS").unwrap_or("1".to_string());

    let http_server = HttpServer::new(move || {
        let cors = middleware::cors();
        App::new()
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    });
    let server_bind = match enable_tls.as_str() {
        "1" => http_server.bind_rustls_0_23(
            ("127.0.0.1", service_port),
            load_rustls_config::load_rustls_config(),
        ),
        _ => http_server.bind(("127.0.0.1", service_port)),
    };
    server_bind?.run().await
}
