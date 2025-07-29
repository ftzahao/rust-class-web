use crate::errors::AppError;
use actix_web::{Result, web};
pub mod redis;

/// 提供用于序列化和反序列化 `chrono::NaiveDateTime` 的字段属性的工具
pub mod serde_timestamp {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

// 提取路径参数
pub fn extract_path_param<T>(param: Result<web::Path<T>>, param_name: &str) -> Result<T, AppError> {
    match param {
        Ok(path) => Ok(path.into_inner()),
        Err(_) => Err(AppError::BadRequest(format!("无效的{param_name}"))),
    }
}
