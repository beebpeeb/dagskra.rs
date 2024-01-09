use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let fmt = "%Y-%m-%d %H:%M:%S";
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, fmt).map_err(serde::de::Error::custom)?;
    Ok(dt)
}
