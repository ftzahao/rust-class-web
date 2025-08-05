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
    pub async fn tracing_init(&self) -> Result<()> {
        let builder = tracing_subscriber::fmt()
            .with_writer(match self.make_writer.to_uppercase().as_str() == "FILE" {
                true => BoxMakeWriter::new(Logger::file_appender(self)),
                false => BoxMakeWriter::new(std::io::stdout),
            })
            .with_ansi(self.show_ansi)
            .with_thread_ids(self.show_thread_ids)
            .with_thread_names(self.show_thread_names)
            .with_file(self.show_file_paths)
            .with_line_number(self.show_line_number)
            .with_level(self.show_level)
            .with_target(self.show_target)
            .with_timer(Logger::message_time_stamp())
            .with_max_level(Logger::max_level(self));
        // 新增：根据 enable_json_formatter 输出 JSON 格式
        if self.enable_json_formatter {
            builder.json().init();
        } else {
            builder.init();
        }
        println!("日志记录器初始化完成");
        Ok(())
    }
}
