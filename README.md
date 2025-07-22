# 这是一个我个人学习练手的 rust web 项目

- 技术：rust + actix-web + sqlite
- 安装 `watchexec-cli`：`cargo install watchexec-cli`
- 开发: `watchexec -w src -r cargo run`
- 部署：

  - 打包: 先执行 `cargo clean` 在 `cargo build`
  - 把 `target/release/rust-class-web` 和 `config/app.toml` 以及依赖的文件放到同一目录下，然后进入该目录
  - 查询 PID: `lsof -i:8001`
  - 假设查询到的 PID 为 79291，停止进程: `kill -9 79291`
  - 如果无权限: `chmod 777 rust-class-web`
  - 运行: `sudo nohup ./rust-class-web &`
  - OK!

- 配置相关：

  - 需要同目录下创建 `config/app.toml` 文件

    以下为 `config/app.toml` 文件内容默认值，所有配置均可以删除或注释

    ```toml
    # 服务器启动配置
    [server]
    # 服务器主机地址
    host = "127.0.0.1"
    # 服务器端口
    port = 8001
    # 启用的 TLS 版本
    enabled_tls = true
    # 证书文件路径
    tls_cert_path = "./cert.pem"
    # 密钥文件路径
    tls_key_path = "./key.pem"

    # 数据库配置
    [db]
    # 数据库 (目前仅支持 sqlite)
    url = "sqlite://./data/db.sqlite?mode=rwc"
    # 最大连接数
    max_connections = 100
    # 最小连接数
    min_connections = 3

    # redis 配置
    [redis]
    # Redis 服务器地址
    # 配置格式参考: https://docs.rs/redis/latest/redis/#connection-parameters
    url = "redis://192.168.64.4:6379"

    # logger 配置
    [logger]
    # 日志记录器的输出方式
    # - "file" | "FILE" 文件输出
    # - "stdout" | "STDOUT" 标准输出
    make_writer = "file"
    # 日志文件路径
    directory = "./data/logs"
    # 日志文件名的前缀
    filename_prefix = "app"
    # 日志文件名的后缀
    filename_suffix = "log"
    # 日志最大级别
    # 可选值包括：
    # - "error" | "ERROR" 错误级别（1级）
    # - "warn" | "WARN" 警告级别（2级）
    # - "info" | "INFO" 信息级别（3级）
    # - "debug" | "DEBUG" 调试级别（4级）
    # - "trace" | "TRACE" 跟踪级别（5级）
    max_level = "debug"
    # 日志文件的滚动策略
    # - "minutely" | "MINUTELY" 每分钟滚动
    # - "hourly" | "HOURLY" 每小时滚动
    # - "daily" | "DAILY" 每天滚动
    # - "never" | "NEVER" 不滚动
    rotation = "minutely"
    # 最大日志文件数，仅对日志文件的滚动策略为 "minutely"、"hourly" 或 "daily" 时有效
    # 当达到最大日志文件数时，最旧的日志文件将被删除
    max_log_files = 30
    # 是否启用 JSON 格式的日志输出
    enable_json_formatter = true
    # 是否显示事件的目标
    show_target = true
    # 是否显示线程 ID
    show_thread_ids = true
    # 是否显示线程名称
    show_thread_names = true
    # 是否显示事件的源代码文件路径
    show_file_paths = true
    # 是否显示事件的源代码行号
    show_line_number = true
    # 是否显示日志等级
    show_level = true
    # 是否显示 ANSI 颜色
    show_ansi = false
    ```
