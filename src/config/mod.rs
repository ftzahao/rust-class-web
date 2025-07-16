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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub tls: Tls,
}

impl Config {
    pub fn new() -> Self {
        dbg!("加载配置");
        let config: Config = Figment::new()
            // 先加载结构体默认值
            .merge(Serialized::defaults(Config::default()))
            // 再加载 config.toml，文件中有的字段会覆盖默认值
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("配置加载失败");
        println!("{:#?}", config);
        config
    }
}
