use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

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
        let url = &self.url;
        let mut client_options = ClientOptions::parse(url).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        client
            .database("admin")
            .run_command(doc! { "ping": 1 })
            .await?;
        println!("MongoDB 连接成功: {url}");
        Ok(client)
    }
}
