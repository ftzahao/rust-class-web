use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

use crate::utils;

/// 服务器启动配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Server {
    /// 服务器主机地址
    pub host: std::net::IpAddr,
    /// 服务器端口
    pub port: u16,
    /// 启用的 TLS 版本
    /// - "default": 不启用 TLS
    /// - "rustls-0_23": 使用 Rustls 0.23
    /// - "openssl": 使用 OpenSSL
    pub enabled_tls: String,
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
            enabled_tls: "default".to_string(),
            tls_cert_path: "cert.pem".to_string(),
            tls_key_path: "key.pem".to_string(),
        }
    }
}

impl Server {
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
        let mut ip_tips: Vec<String> = vec![];
        let ip = utils::local_ip();
        match self.enabled_tls.as_str() {
            "rustls-0_23" => {
                ip_tips.push(format!("➜ Network: https://{ip}:{}", self.port));
            }
            "openssl" => {
                ip_tips.push(format!("➜ Network: https://{ip}:{}", self.port));
            }
            _ => {
                ip_tips.push(format!("➜ Local:   http://localhost:{}", self.port));
                ip_tips.push(format!("➜ Local:   http://127.0.0.1:{}", self.port));
                if ip != "127.0.0.1" {
                    ip_tips.push(format!("➜ Network: http://{ip}:{}", self.port));
                }
            }
        }
        for tip in ip_tips.iter() {
            println!("{tip}");
        }
    }
}
