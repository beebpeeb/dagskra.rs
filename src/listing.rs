use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::date_format;

const SUFFIX: &str = " e.";

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Listing {
    description: String,
    live: bool,
    #[serde(rename = "startTime", with = "date_format")]
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
