# 这是一个我个人学习练手的 rust web 项目

- 技术：rust + actix-web + sqlite + rustls
- 部署：
  - 打包: `cargo make build`
  - 把 `target/release/rust-class-web` 和 `.env` 以及依赖的文件放到同一目录下，然后进入该目录
  - 查询 PID: `lsof -i:8001`
  - 假设查询到的 PID 为 79291，停止进程: `kill -9 79291`
  - 如果无权限: `chmod 777 rust-class-web`
  - 运行: `sudo nohup ./rust-class-web &`
  - OK!
- 配置相关：

  - 需要同目录下创建 `.env.toml` 文件
    - `LOG_LEVEL = "debug"` 配置日志级别
    - `RUST_LOG = "debug"` 配置日志级别
    - `RUST_BACKTRACE = "1"` 程序错误时出现明显的错误；`RUST_BACKTRACE = "full"` 程序错误时出现更加详细的错误；
    - `DATABASE_URL = "./data/db.sqlite"` sqlite 文件位置, 文件名默认 `db.sqlite`
    - `MAX_CONNECTIONS = 100` 数据库最大连接限制
    - `MIN_CONNECTIONS = 3` 数据库最小连接限制
    - `SERVICE_PORT = 8001` 服务启动端口
    - `ENABLE_TLS = 1` 启用 TLS。1: 启用, 0: 禁用
    - `TLS_CERT_PATH = "./cert.pem"` TLS 证书路径
    - `TLS_KEY_PATH = "./key.pem"` TLS 密钥路径

- TLS 证书：
  - 使用 [`mkcert`](https://github.com/FiloSottile/mkcert) 生成证书
  - 生成证书：`mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost`
  - 安装并信任它：`mkcert -install`
