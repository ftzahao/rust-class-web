/// 数据库配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Db {
    /// 数据库类型
    /// - "sqlite": SQLite 数据库
    pub db_type: String,
    /// 数据库路径
    pub path: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
}
impl Default for Db {
    fn default() -> Self {
        Db {
            db_type: "sqlite".to_string(),
            path: "./data/db.sqlite".to_string(),
            max_connections: 100,
            min_connections: 3,
        }
    }
}
