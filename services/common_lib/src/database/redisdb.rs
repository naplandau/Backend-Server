use redis::{Client, RedisError};
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}
impl RedisConfig {
    fn build_config(self) -> String {
        let url = "redis://".to_owned() + self.host.as_str() + "/";
        url
    }
    pub fn with_file(config_str: &str) -> Self {
        let result: RedisConfig = serde_json::from_str(config_str).unwrap();
        result
    }
}
pub struct RedisClient {
    pub conn: redis::aio::Connection
}
pub struct RedisFactory;
impl RedisFactory {
    pub async fn with_config(config_str: &str) -> Result<RedisClient, RedisError> {
        let config = RedisConfig::with_file(config_str).build_config();
        let client = Client::open(config).unwrap().get_async_connection().await?;
        Ok(RedisClient{conn:client})
    }
}