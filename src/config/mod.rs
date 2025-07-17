mod db;
mod server;
mod tls;

use db::Db;
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

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub server: Server,
    pub db: Db,
    pub tls: Tls,
}

impl Config {
    /// 创建一个新的配置实例
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
}

impl Config {
    /// 获取 OpenSSL 的 SslAcceptorBuilder
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

impl Config {
    /// 获取数据库连接 URL
    pub fn database_url(&self) -> String {
        format!("{}://{}?mode=rwc", self.db.db_type, self.db.path)
    }
    /// 初始化数据库连接池
    pub async fn init_db(config: &Config) -> DatabaseConnection {
        let url = config.database_url();
        println!("正在连接数据库: {url}");

        let mut opt = ConnectOptions::new(url);
        opt.max_connections(config.db.max_connections)
            .min_connections(config.db.min_connections)
            .sqlx_logging(true);
        let db = Database::connect(opt).await.unwrap();
        create_db_table(db.clone()).await; // 确保数据库表存在
        println!("数据库连接成功");
        db
    }
}

/// 检查数据库的完整性，不完整的部分给予补充
async fn create_db_table(pool: DatabaseConnection) {
    pool.execute_unprepared("CREATE TABLE IF NOT EXISTS users(
        id          INTEGER primary key AUTOINCREMENT not null,
        name        text                              not null,
        email       char(20) UNIQUE                   not null,
        pass_word   char(65)                          not null,                                        -- 'passwd hash'
        create_time datetime                          not null default (datetime('now', 'localtime')), -- 'create datetime'
        update_time datetime                          not null default (datetime('now', 'localtime')), -- 'update datetime'
        status      char(10)                          not null default 'normal'                        -- comment 'status: normal, blocked, deleted'
    )").await.unwrap();
}
