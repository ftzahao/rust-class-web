use anyhow::Result;
use redis::{Client, RedisError};

/// Redis 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Redis {
    /// Redis 服务器地址
    pub url: String,
}
impl Default for Redis {
    fn default() -> Self {
        Redis {
            url: "redis://127.0.0.1:6379".to_string(),
        }
    }
}

impl Redis {
    /// 访问 Redis 服务器
    pub async fn client(&self) -> Result<Client, RedisError> {
        let client = Client::open(self.url.clone())?;
        Ok(client)
    }
}
