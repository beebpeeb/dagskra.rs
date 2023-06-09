use chrono::NaiveDateTime;
use serde::{de::Error, Deserialize, Deserializer};
use std::error::Error as StdError;

const SUFFIX: &str = " e.";

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Listing {
    description: String,
    live: bool,
    #[serde(deserialize_with = "deserialize_datetime", rename = "startTime")]
    start_time: NaiveDateTime,
    title: String,
}

impl Listing {
    pub fn date(&self) -> String {
        self.start_time.format("%d.%m.%Y").to_string()
    }

    pub fn details(&self) -> &str {
        self.description.trim().trim_end_matches(SUFFIX)
    }

    pub fn has_details(&self) -> bool {
        !self.description.trim().is_empty()
    }

    pub fn is_live(&self) -> bool {
        self.live
    }

    pub fn is_repeat(&self) -> bool {
        self.description.trim().ends_with(SUFFIX)
    }

    pub fn time(&self) -> String {
        self.start_time.format("%H:%M").to_string()
    }

    pub fn title(&self) -> &str {
        self.title.trim()
    }
}

#[derive(Deserialize)]
struct Response {
    results: Vec<Listing>,
}

pub async fn fetch_listings() -> Result<Vec<Listing>, Box<dyn StdError>> {
    let url = "https://apis.is/tv/ruv";
    let res: Response = reqwest::get(url).await?.json().await?;
    Ok(res.results)
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let fmt = "%Y-%m-%d %H:%M:%S";
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, fmt).map_err(Error::custom)?;
    Ok(dt)
}
