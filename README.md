# 这是一个我个人学习练手的 rust web 项目

- 技术：rust + actix-web + sqlite
- 开发: `cargo run`
- 部署：

  - 打包: `cargo make build`
  - 把 `target/release/rust-class-web` 和 `config.toml` 以及依赖的文件放到同一目录下，然后进入该目录
  - 查询 PID: `lsof -i:8001`
  - 假设查询到的 PID 为 79291，停止进程: `kill -9 79291`
  - 如果无权限: `chmod 777 rust-class-web`
  - 运行: `sudo nohup ./rust-class-web &`
  - OK!

- 配置相关：

  - 需要同目录下创建 `config.toml` 文件

    ```toml
    #
    [server]
    host = "0.0.0.0" # 监听地址
    port = 8001 # 监听端口

    [database]
    db_type = "sqlite" # 数据库类型
    path = "./data/db.sqlite" # 数据库路径
    max_connections = 100 # 最大连接数
    min_connections = 3 # 最小连接数

    [tls]
    # TLS类型: "rustls-0_23" | "openssl" | "default"
    # rustls-0_23: 使用 rustls 0.23 版本，可使用 mkcert 生成证书和密钥文件, 以 https 方式访问
    # openssl: 使用 openssl 生成证书和密钥文件, 以 https 方式访问
    # default: 默认不加密, 以 http 方式访问
    enabled = "default"
    cert_path = "./cert.pem" # TLS证书路径
    key_path = "./key.pem" # TLS密钥路径
    ```
