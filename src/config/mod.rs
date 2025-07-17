mod database;
mod server;
mod tls;

use database::Database;
use server::Server;
use tls::Tls;

use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub tls: Tls,
}

impl Config {
    pub fn new() -> Self {
        debug!("加载配置");
        let config: Config = Figment::new()
            // 先加载结构体默认值
            .merge(Serialized::defaults(Config::default()))
            // 再加载 config.toml，文件中有的字段会覆盖默认值
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("配置加载失败");
        debug!("{:#?}", config);
        config
    }
    pub fn openssl_builder(&self) -> SslAcceptorBuilder {
        // load TLS keys
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(self.tls.key_path.clone(), SslFiletype::PEM)
            .unwrap();
        builder
            .set_certificate_chain_file(self.tls.cert_path.clone())
            .unwrap();
        builder
    }
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
        let cert_chain = CertificateDer::pem_file_iter(self.tls.cert_path.clone())
            .unwrap()
            .flatten()
            .collect();

        let key_der = PrivateKeyDer::from_pem_file(self.tls.key_path.clone())
            .expect("Could not locate PKCS 8 private keys.");

        // set up TLS config options
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)
            .unwrap()
    }
}
