use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub host: String, // 服务器主机地址
    pub port: u16,    // 服务器端口
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub db_type: String,      // 数据库类型: "sqlite"
    pub path: String,         // 数据库路径
    pub max_connections: u32, // 数据库最大连接数
    pub min_connections: u32, //
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tls {
    pub enabled: String,   // TLS类型: "rustls-0_23" 或 "NoTLS"
    pub cert_path: String, // 证书路径
    pub key_path: String,  // 密钥路径
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: Server,     // Server configuration
    pub database: Database, // Database configuration
    pub tls: Tls,           // TLS configuration
}

impl Config {
    pub fn new() -> Self {
        let content = read_to_string("config.toml").expect("Failed to read config file");
        let config: toml::Value = toml::from_str(&content).expect("Failed to parse config file");

        Config {
            server: Server {
                host: config["server"]["host"]
                    .as_str()
                    .unwrap_or("127.0.0.1")
                    .into(),
                port: config["server"]["port"]
                    .as_integer()
                    .unwrap_or(8081)
                    .try_into()
                    .unwrap(),
            },
            database: Database {
                db_type: config["database"]["db_type"]
                    .as_str()
                    .unwrap_or("sqlite")
                    .into(),
                path: config["database"]["path"]
                    .as_str()
                    .unwrap_or("db.sqlite")
                    .into(),
                max_connections: config["database"]["max_connections"]
                    .as_integer()
                    .unwrap_or(100)
                    .try_into()
                    .unwrap(),
                min_connections: config["database"]["min_connections"]
                    .as_integer()
                    .unwrap_or(3)
                    .try_into()
                    .unwrap(),
            },
            tls: Tls {
                enabled: config["tls"]["enabled"]
                    .as_str()
                    .unwrap_or("rustls-0_23")
                    .into(),
                cert_path: config["tls"]["cert_path"]
                    .as_str()
                    .unwrap_or("cert.pem")
                    .into(),
                key_path: config["tls"]["key_path"]
                    .as_str()
                    .unwrap_or("key.pem")
                    .into(),
            },
        }
    }
}
