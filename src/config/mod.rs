mod db;
mod logger;
mod server;

use db::Db;
use logger::Logger;
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
}

impl Config {
    /// 创建一个新的配置实例
    pub fn new() -> Self {
        println!("加载配置文件 config.toml");
        let config: Config = Figment::new()
            // 先加载结构体默认值
            .merge(Serialized::defaults(Config::default()))
            // 再加载 config.toml，文件中有的字段会覆盖默认值
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("配置加载失败");
        println!("配置加载完成: {:#?}", config);
        config
    }
}
