use askama::Template;
use axum::{routing, Router};
use shuttle_axum::ShuttleAxum;
use tracing::info;

use dagskra::{fetch_listings, Listings};

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    info!("starting");
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
    listings: Listings,
}

async fn listings() -> ListingsTemplate {
    info!("fetching schedule data");
    let listings: Listings = fetch_listings().await.unwrap_or_default();
    let date: String = listings.first().map_or("".to_string(), |l| l.date());
    ListingsTemplate { date, listings }
}
