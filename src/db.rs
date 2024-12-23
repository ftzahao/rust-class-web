use sqlx::{
    query,
    sqlite::{SqlitePool, SqlitePoolOptions},
};
use std::fs;
use std::path::Path;

pub async fn init_db() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or("./data/db.sqlite".to_string());
    let max_connections = std::env::var("MAX_CONNECTIONS")
        .unwrap_or("100".to_string())
        .parse::<u32>()
        .unwrap_or(100);
    let min_connections = std::env::var("MIN_CONNECTIONS")
        .unwrap_or("3".to_string())
        .parse::<u32>()
        .unwrap_or(3);

    create_db_file(&database_url);

    let sqlite = match SqlitePoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect(&format!("sqlite://{}", &database_url))
        .await
    {
        Ok(pool) => {
            println!("连接数据库成功!");
            pool
        }
        Err(err) => {
            println!("连接数据库失败: {:?}", err);
            std::process::exit(1);
        }
    };
    create_db_table(sqlite.clone()).await;
    sqlite
}

/// 数据库文件不存在则创建
fn create_db_file(database_url: &str) {
    if !Path::new(database_url).exists() {
        // 考虑到某个路径下没有目录的行为，先创建目录，在创建文件
        fs::create_dir_all(Path::new(database_url).parent().unwrap()).unwrap();
        fs::File::create(database_url).unwrap();
    }
}

/// 检查数据库的完整性，不完整的部分给予补充
async fn create_db_table(pool: SqlitePool) {
    println!("{:#?}", pool);
    // 检查数据库是否有表 `users`，没有则创建
    query("CREATE TABLE IF NOT EXISTS users(
        id          INTEGER primary key AUTOINCREMENT not null,
        name        text                              not null,
        email       char(20) UNIQUE                   not null,
        pass_word   char(65)                          not null,                                        -- 'passwd hash'
        create_time datetime                          not null default (datetime('now', 'localtime')), -- 'create datetime'
        update_time datetime                          not null default (datetime('now', 'localtime')), -- 'update datetime'
        status      char(10)                          not null default 'normal'                        -- comment 'status: normal, blocked, deleted'
    )").execute(&pool)
        .await
        .unwrap();
}
