use mongodb::{Client, bson::doc};

/// MongoDB 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Mongodb {
    /// MongoDB 服务器地址
    pub url: String,
}
impl Default for Mongodb {
    fn default() -> Self {
        Mongodb {
            url: "mongodb://192.168.64.2:27017".to_string(),
        }
    }
}

impl Mongodb {
    /// 访问 MongoDB 服务器
    pub async fn client(&self) -> Result<Client, mongodb::error::Error> {
        let client = Client::with_uri_str(self.url.clone()).await?;
        // 使用 ping 命令检测连接
        client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .await?;

        println!("Successfully connected to MongoDB at {}", self.url);
        Ok(client)
    }
}
