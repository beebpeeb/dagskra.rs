use reqwest::get;
use serde::Deserialize;
use std::error::Error;

use crate::Listing;

#[derive(Deserialize)]
struct Response {
    results: Vec<Listing>,
}

pub async fn fetch_listings() -> Result<Vec<Listing>, Box<dyn Error>> {
    let url = "https://apis.is/tv/ruv";
    let res: Response = get(url).await?.json().await?;
    Ok(res.results)
}
