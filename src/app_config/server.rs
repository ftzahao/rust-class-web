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
    pub enabled_tls: bool,
    /// 证书路径
    pub tls_cert_path: String,
    /// 密钥路径
    pub tls_key_path: String,
}
impl Default for Server {
    fn default() -> Self {
        Server {
            host: "127.0.0.1".parse().unwrap(),
            port: 8001,
            enabled_tls: false,
            tls_cert_path: "cert.pem".to_string(),
            tls_key_path: "key.pem".to_string(),
        }
    }
}

impl Server {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn rustls_config(&self) -> ServerConfig {
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
        let use_tls = self.enabled_tls;
        println!("{use_tls} TLS enabled");
        let scheme = if use_tls { "https" } else { "http" };
        let ip = utils::local_ip();
        let mut tips = Vec::new();

        if use_tls {
            tips.push(format!("➜ Network: {scheme}://{ip}"));
        } else {
            tips.push(format!("➜ Local:   {scheme}://localhost"));
            tips.push(format!("➜ Local:   {scheme}://127.0.0.1"));
            if ip != "127.0.0.1" && use_tls {
                tips.push(format!("➜ Network: {scheme}://{ip}"));
            }
        }

        for tip in tips {
            println!("{tip}:{}", self.port);
        }
    }
}
