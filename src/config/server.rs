use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

use crate::utils;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub enum EnabledTls {
    Disabled,
    Enabled,
    Mode(String),
}

impl Default for EnabledTls {
    fn default() -> Self {
        EnabledTls::Disabled
    }
}

impl EnabledTls {
    /// 是否启用 TLS
    pub fn use_tls(&self) -> bool {
        !matches!(self, EnabledTls::Disabled)
    }
    /// 返回使用的 TLS 方法（"none"/"openssl"/自定义模式）
    pub fn method(&self) -> &str {
        match self {
            EnabledTls::Disabled => "none",
            EnabledTls::Enabled => "openssl",
            EnabledTls::Mode(mode) => mode.as_str(),
        }
    }
}

impl<'de> Deserialize<'de> for EnabledTls {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        Ok(match v {
            Value::Bool(true) => EnabledTls::Enabled,
            Value::Bool(false) => EnabledTls::Disabled,
            Value::String(s) => match s.as_str() {
                "true" => EnabledTls::Enabled,
                "false" => EnabledTls::Disabled,
                other => EnabledTls::Mode(other.to_string()),
            },
            _ => EnabledTls::Disabled,
        })
    }
}

/// 服务器启动配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Server {
    /// 服务器主机地址
    pub host: std::net::IpAddr,
    /// 服务器端口
    pub port: u16,
    /// 启用的 TLS 版本
    /// - false: 不启用 TLS
    /// - "rustls-0_23": 使用 Rustls 0.23
    /// - true | "openssl": 使用 OpenSSL
    pub enabled_tls: EnabledTls,
    /// 证书路径
    pub tls_cert_path: String,
    /// 密钥路径
    pub tls_key_path: String,
}
impl Default for Server {
    fn default() -> Self {
        Server {
            host: "0.0.0.0".parse().unwrap(),
            port: 8001,
            enabled_tls: EnabledTls::Disabled,
            tls_cert_path: "cert.pem".to_string(),
            tls_key_path: "key.pem".to_string(),
        }
    }
}

impl Server {
    pub fn addr(&self) -> (std::net::IpAddr, u16) {
        (self.host, self.port)
    }
    /// 获取 OpenSSL 的 SslAcceptorBuilder
    pub fn openssl_builder(&self) -> SslAcceptorBuilder {
        // load TLS keys
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(self.tls_key_path.clone(), SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(self.tls_cert_path.clone())
            .unwrap();
        builder
    }
    /// 获取 Rustls 的 ServerConfig
    pub fn rustls_config(&self) -> ServerConfig {
        // to create a self-signed temporary cert for testing:
        // `mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost`
        // to install mkcert CA in the system trust store, run:
        // `mkcert -install`
        // to uninstall mkcert CA from the system trust store, run:
        // `mkcert -uninstall`
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .unwrap();

        // load TLS key/cert files
        let cert_chain = CertificateDer::pem_file_iter(self.tls_cert_path.clone())
            .unwrap()
            .flatten()
            .collect();

        let key_der = PrivateKeyDer::from_pem_file(self.tls_key_path.clone())
            .expect("Could not locate PKCS 8 private keys.");

        // set up TLS config options
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)
            .unwrap()
    }
    /// 打印服务器启动的地址
    pub fn print_server_startup_address(&self) {
        let use_tls = self.enabled_tls.use_tls();
        let scheme = if use_tls { "https" } else { "http" };
        let ip = utils::local_ip();
        let mut tips = Vec::new();

        if use_tls {
            tips.push(format!("➜ Network: {scheme}://{ip}"));
        } else {
            tips.push(format!("➜ Local:   {scheme}://localhost"));
            tips.push(format!("➜ Local:   {scheme}://127.0.0.1"));
            if ip != "127.0.0.1" {
                tips.push(format!("➜ Network: {scheme}://{ip}"));
            }
        }

        for tip in tips {
            println!("{tip}:{}", self.port);
        }
    }
}
