#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Db {
    pub db_type: String,      // 数据库类型: "sqlite"
    pub path: String,         // 数据库路径
    pub max_connections: u32, // 数据库最大连接数
    pub min_connections: u32, // 数据库最小连接数
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
