use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    Ok(dt)
}
