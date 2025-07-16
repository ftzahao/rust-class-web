#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Server {
    pub host: std::net::IpAddr, // 服务器主机地址
    pub port: u16,              // 服务器端口
}
impl Default for Server {
    fn default() -> Self {
        Server {
            host: "0.0.0.0".parse().unwrap(),
            port: 8081,
        }
    }
}
