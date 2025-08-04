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

  - 需要同目录下创建 [config/app.toml](./config/app.toml) 文件
