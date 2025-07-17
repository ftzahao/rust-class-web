use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn tracing_init() -> WorkerGuard {
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

    let rust_log = match std::env::var("RUST_LOG") {
        Ok(directives) => directives,
        Err(_) => {
            eprintln!("RUST_LOG environment variable is not set, defaulting to 'trace'");
            tracing::Level::TRACE.to_string()
        }
    };
    let directives = match rust_log.as_str() {
        "trace" | "TRACE" => tracing::Level::TRACE.to_string(),
        "debug" | "DEBUG" => tracing::Level::DEBUG.to_string(),
        "info" | "INFO" => tracing::Level::INFO.to_string(),
        "warn" | "WARN" => tracing::Level::WARN.to_string(),
        "error" | "ERROR" => tracing::Level::ERROR.to_string(),
        _ => {
            eprintln!("无效的RUST_LOG值: '{rust_log}'. 默认为 'DEBUG'");
            tracing::Level::DEBUG.to_string()
        }
    };

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(rolling_file_appender);
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new(directives))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    guard
}
