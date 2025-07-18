/// 服务器启动配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Server {
    /// 服务器主机地址
    pub host: std::net::IpAddr,
    /// 服务器端口
    pub port: u16,
}
impl Default for Server {
    fn default() -> Self {
        Server {
            host: "0.0.0.0".parse().unwrap(),
            port: 8001,
        }
    }
}
