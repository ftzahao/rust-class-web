use anyhow::Result;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub async fn tracing_init() -> Result<()> {
    // 将所有“跟踪”事件记录到前缀为“调试”的文件中。因为这些文件将非常频繁地写入，每分钟滚动日志文件。
    let debug_file = rolling::minutely("./data/logs", "debug");
    // 将警告和错误日志记录到一个单独的文件中。由于我们预计这些事件发生的频率较低，因此每天滚动该文件。
    let warn_file = rolling::daily("./data/logs", "warnings").with_max_level(tracing::Level::WARN);

    let all_files = debug_file.and(warn_file);

    tracing_subscriber::fmt()
        .with_writer(all_files)
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(false)
        // .json() // 取消此行以启用JSON输出
        .with_max_level(tracing::Level::TRACE)
        .init();
    Ok(())
}
