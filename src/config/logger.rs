use crate::config::Config;
use anyhow::Result;
use time::{
    UtcOffset,
    format_description::{self, BorrowedFormatItem},
};
use tracing_appender::rolling;
use tracing_subscriber::fmt::{time::OffsetTime, writer::BoxMakeWriter};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Logger {
    /// 日志记录器的输出方式
    /// - "file" | "FILE" 文件输出
    /// - "stdout" | "STDOUT" 标准输出
    pub make_writer: String,
    /// 日志文件路径
    pub directory: String,
    /// 日志文件名的前缀
    pub filename_prefix: String,
    /// 日志文件名的后缀
    pub filename_suffix: String,
    /// 日志最大级别
    pub max_level: String,
    /// 最大日志文件数
    pub max_log_files: usize,
    /// 是否启用 JSON 格式的日志输出
    pub enable_json_formatter: bool,
    /// 日志文件的滚动策略
    /// - "minutely" | "MINUTELY" 每分钟滚动
    /// - "hourly" | "HOURLY" 每小时滚动
    /// - "daily" | "DAILY" 每天滚动
    /// - "never" | "NEVER" 不滚动
    pub rotation: String,
    /// 是否显示事件的目标
    pub show_target: bool,
    /// 是否显示线程 ID
    pub show_thread_ids: bool,
    /// 是否显示线程名称
    pub show_thread_names: bool,
    /// 是否显示事件的源代码文件路径
    pub show_file_paths: bool,
    /// 是否显示事件的源代码行号
    pub show_line_number: bool,
    /// 是否显示日志等级
    pub show_level: bool,
    /// 是否显示 ANSI 颜色
    pub show_ansi: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            make_writer: String::from("file"),
            directory: String::from("./data/logs"),
            filename_prefix: String::from("app"),
            filename_suffix: String::from("log"),
            max_level: String::from("info"),
            max_log_files: 30,
            enable_json_formatter: false,
            rotation: String::from("minutely"),
            show_target: false,
            show_thread_ids: true,
            show_thread_names: true,
            show_file_paths: true,
            show_line_number: true,
            show_level: true,
            show_ansi: false,
        }
    }
}
impl Logger {
    pub fn file_appender(&self) -> rolling::RollingFileAppender {
        rolling::RollingFileAppender::builder()
            .rotation(match self.rotation.to_uppercase().as_str() {
                "MINUTELY" => rolling::Rotation::MINUTELY,
                "HOURLY" => rolling::Rotation::HOURLY,
                "DAILY" => rolling::Rotation::DAILY,
                _ => rolling::Rotation::NEVER,
            })
            .filename_prefix(self.filename_prefix.clone())
            .filename_suffix(self.filename_suffix.clone())
            .max_log_files(self.max_log_files)
            .build(self.directory.clone())
            .expect("Failed to create file appender")
    }
    pub fn message_time_stamp() -> OffsetTime<Vec<BorrowedFormatItem<'static>>> {
        let timer = OffsetTime::new(
            UtcOffset::current_local_offset().unwrap_or_else(|_| UtcOffset::UTC),
            format_description::parse("[offset_hour sign:mandatory]:[offset_minute] [year]-[month]-[day] [hour]:[minute]:[second]")
                .expect("Failed to parse time format description"),
        );
        timer
    }
    pub fn max_level(&self) -> tracing::Level {
        match self.max_level.to_uppercase().as_str() {
            "TRACE" => tracing::Level::TRACE,
            "DEBUG" => tracing::Level::DEBUG,
            "INFO" => tracing::Level::INFO,
            "WARN" => tracing::Level::WARN,
            "ERROR" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        }
    }
}

impl Config {
    pub async fn tracing_init(&self) -> Result<()> {
        println!("日志记录器正在初始化...");
        let logger = &self.logger;
        let Logger {
            make_writer,
            directory: _,
            filename_prefix: _,
            filename_suffix: _,
            max_level: _,
            max_log_files: _,
            enable_json_formatter: _,
            rotation: _,
            show_target,
            show_thread_ids,
            show_thread_names,
            show_file_paths,
            show_line_number,
            show_level,
            show_ansi,
        } = logger;
        let builder = tracing_subscriber::fmt()
            .with_ansi(*show_ansi)
            .with_thread_ids(*show_thread_ids)
            .with_thread_names(*show_thread_names)
            .with_file(*show_file_paths)
            .with_line_number(*show_line_number)
            .with_level(*show_level)
            .with_target(*show_target)
            .with_timer(Logger::message_time_stamp())
            .with_max_level(Logger::max_level(logger));
        let builder = if make_writer.to_uppercase().as_str() == "FILE" {
            builder.with_writer(BoxMakeWriter::new(Logger::file_appender(logger)))
        } else {
            builder.with_writer(BoxMakeWriter::new(std::io::stdout))
        };
        // 新增：根据 enable_json_formatter 输出 JSON 格式
        if logger.enable_json_formatter {
            builder.json().init();
        } else {
            builder.init();
        }

        Ok(())
    }
}
