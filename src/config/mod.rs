pub mod db;
pub mod logger;
pub mod redis;
pub mod server;

use db::Db;
use logger::Logger;
use redis::Redis;
use server::Server;

use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct Config {
    /// 服务器配置
    pub server: Server,
    /// 数据库配置
    pub db: Db,
    /// 日志配置
    pub logger: Logger,
    /// Redis 配置
    pub redis: Redis,
}

impl Config {
    /// 创建一个新的配置实例
    pub fn new() -> Self {
        println!("加载配置文件 config.toml");
        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("配置加载失败");
        println!("配置加载完成");
        config
    }
}
