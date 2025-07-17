use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

use crate::state::{CARGO_PKG_NAME, CARGO_PKG_VERSION};

pub fn tracing_init() -> WorkerGuard {
    LogTracer::init().expect("Unable to setup log tracer!");

    let filename_prefix = format!("[{CARGO_PKG_NAME}].[{CARGO_PKG_VERSION}].log");

    let rolling_file_appender =
        RollingFileAppender::new(Rotation::HOURLY, "./data/logs", filename_prefix.clone());

    let directives = match std::env::var("RUST_LOG") {
        Ok(directives) => match directives.as_str() {
            "trace" | "TRACE" => tracing::Level::TRACE.to_string(),
            "debug" | "DEBUG" => tracing::Level::DEBUG.to_string(),
            "info" | "INFO" => tracing::Level::INFO.to_string(),
            "warn" | "WARN" => tracing::Level::WARN.to_string(),
            "error" | "ERROR" => tracing::Level::ERROR.to_string(),
            _ => {
                eprintln!("RUST_LOG 环境变量无效: '{directives}'. 默认为 'DEBUG'");
                tracing::Level::DEBUG.to_string()
            }
        },
        Err(_) => {
            eprintln!("RUST_LOG 环境变量无效, 默认为 'trace'");
            tracing::Level::TRACE.to_string()
        }
    };

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(rolling_file_appender);
    let bunyan_formatting_layer = BunyanFormattingLayer::new(filename_prefix, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new(directives))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    guard
}
