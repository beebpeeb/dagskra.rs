use askama::Template;
use axum::{routing, Router};
use shuttle_axum::ShuttleAxum;

use dagskra::{fetch_listings, Listing};

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    tracing::info!("Starting app");
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/_listings", routing::get(listings));
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
    tracing::info!("Fetching schedule data");
    let listings = fetch_listings().await.unwrap_or_default();
    let date = listings
        .first()
        .map_or_else(|| "".to_string(), |listing| listing.date());
    ListingsTemplate { date, listings }
}
