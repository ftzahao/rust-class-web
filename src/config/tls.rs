#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Tls {
    pub enabled: String,   // TLS类型: "rustls-0_23" | "openssl" | "default"
    pub cert_path: String, // 证书路径
    pub key_path: String,  // 密钥路径
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
