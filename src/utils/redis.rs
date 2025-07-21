use crate::state::AppState;
use anyhow::Result;
use redis::Commands;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// 从Redis获取一个值
pub async fn fetch_redis<T: DeserializeOwned>(state: &AppState, key: &str) -> Result<Option<T>> {
    let mut conn = state.redis_client.get_connection().unwrap();
    let data: Option<String> = conn.get(key)?;

    match data {
        Some(json) => {
            let parsed: T = serde_json::from_str(&json)?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

/// 在Redis中设置一个值
pub async fn set_redis<T: Serialize>(
    state: &AppState,
    key: &str,
    value: &T,
    ttl: Option<u64>,
) -> Result<()> {
    let ttl = ttl.unwrap_or(300) as u64; // 默认TTL为300秒
    let value = serde_json::to_string(value)?;
    let mut conn = state.redis_client.get_connection().unwrap();
    let _: () = conn.set_ex(key, value, ttl)?;
    Ok(())
}
