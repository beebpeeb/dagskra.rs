use askama::Template;
use axum::{routing::get, Router};
use shuttle_axum::ShuttleAxum;
use tracing::info;

use dagskra::{fetch_listings, Listing};

#[shuttle_runtime::main]
async fn axum_app() -> ShuttleAxum {
    info!("Starting app");
    let router = Router::new()
        .route("/", get(index))
        .route("/_listings", get(listings));
    Ok(router.into())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

async fn index() -> IndexTemplate {
    let title = "Dagskrá RÚV";
    IndexTemplate { title }
}

#[derive(Template)]
#[template(path = "_listings.html")]
struct ListingsTemplate {
    date: String,
    listings: Vec<Listing>,
}

async fn listings() -> ListingsTemplate {
    info!("Fetching schedule data");
    let listings = fetch_listings().await.unwrap_or_default();
    let date = listings
        .first()
        .map_or_else(|| "".to_string(), |listing| listing.date());
    ListingsTemplate { date, listings }
}
