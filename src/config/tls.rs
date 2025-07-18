/// TLS配置模块
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Tls {
    /// 启用的 TLS 版本
    /// - "default": 不启用 TLS
    /// - "rustls-0_23": 使用 Rustls 0.23
    /// - "openssl": 使用 OpenSSL
    pub enabled: String,
    /// 证书路径
    pub cert_path: String,
    /// 密钥路径
    pub key_path: String,
}
impl Default for Tls {
    fn default() -> Self {
        Tls {
            enabled: "default".to_string(),
            cert_path: "cert.pem".to_string(),
            key_path: "key.pem".to_string(),
        }
    }
}
