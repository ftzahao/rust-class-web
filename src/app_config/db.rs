use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

/// 数据库配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Db {
    /// 数据库路径
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
}
impl Default for Db {
    fn default() -> Self {
        Db {
            url: "sqlite://./data/db.sqlite?mode=rwc".to_string(),
            max_connections: 100,
            min_connections: 3,
        }
    }
}

impl Db {
    /// 初始化数据库连接池
    pub async fn init_db(&self) -> DatabaseConnection {
        let url = &self.url;
        println!("正在连接数据库: {}", url);

        let mut opt = ConnectOptions::new(url);
        opt.max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .sqlx_logging(true);
        let db = Database::connect(opt).await.unwrap();
        create_db_table(db.clone()).await; // 确保数据库表存在
        println!("数据库连接成功");
        db
    }
}
/// 检查数据库的完整性，不完整的部分给予补充
async fn create_db_table(pool: DatabaseConnection) {
    // 新增用户表
    pool.execute_unprepared("CREATE TABLE IF NOT EXISTS users(
        id          INTEGER primary key AUTOINCREMENT not null,
        name        text                              not null,
        email       char(20) UNIQUE                   not null,
        pass_word   char(65)                          not null,                                        -- 'passwd hash'
        create_time datetime                          not null default (datetime('now', 'localtime')), -- 'create datetime'
        update_time datetime                          not null default (datetime('now', 'localtime')), -- 'update datetime'
        status      char(10)                          not null default 'normal'                        -- comment 'status: normal, blocked, deleted'
    )").await.unwrap();

    // 新增设备管理表
    pool.execute_unprepared("CREATE TABLE IF NOT EXISTS devices(
        id          INTEGER primary key AUTOINCREMENT not null,           -- 唯一id
        user_id     INTEGER                           not null,           -- users表中的id
        token       text                              not null,           -- 设备token
        create_time datetime                          not null default (datetime('now', 'localtime')), -- 创建时间
        update_time datetime                          not null default (datetime('now', 'localtime')), -- 更新时间
        name        text                              null,           -- 设备名称
        FOREIGN KEY(user_id) REFERENCES users(id)
    )").await.unwrap();
}
