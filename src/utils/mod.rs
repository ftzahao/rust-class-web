pub mod redis;

/// 提供用于序列化和反序列化 `chrono::NaiveDateTime` 的字段属性的工具
pub mod serde_timestamp {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    /// 将 `NaiveDateTime` 序列化为字符串（"YYYY-MM-DD HH:MM:SS"）格式。
    ///
    /// # 参数
    /// - `date`: 需要序列化的 `NaiveDateTime` 引用。
    /// - `serializer`: Serde 的序列化器。
    ///
    /// # 返回
    /// 返回序列化后的字符串结果。
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    /// 从字符串（"YYYY-MM-DD HH:MM:SS"）格式反序列化为 `NaiveDateTime`。
    ///
    /// # 参数
    /// - `deserializer`: Serde 的反序列化器。
    ///
    /// # 返回
    /// 返回反序列化后的 `NaiveDateTime`。
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
    }
}
