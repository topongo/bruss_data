pub(crate) mod date_time {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(DateTime::from_timestamp(i64::deserialize(deserializer)?, 0).unwrap())
    }
    
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        date.timestamp().serialize(serializer)
    }
}
