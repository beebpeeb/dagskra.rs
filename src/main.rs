use askama::Template;
use axum::{routing::get, Router};
use axum_htmx::HxRequestGuardLayer;
use chrono::Utc;
use shuttle_axum::ShuttleAxum;
use tracing::info;

use dagskra::{fetch_listings, Listing};

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    info!("Hleður...");
    let htmx_router = Router::new()
        .route("/htmx/listings", get(listings))
        .layer(HxRequestGuardLayer::default());
    let general_router = Router::new().route("/", get(index));
    let router = Router::new().merge(htmx_router).merge(general_router);
    Ok(router.into())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    date: String,
    title: &'static str,
}

async fn index() -> IndexTemplate {
    let date = Utc::now().format("%d.%m.%Y").to_string();
    let title = "Dagskrá RÚV";
    IndexTemplate { date, title }
}

#[derive(Template)]
#[template(path = "htmx/listings.html")]
struct ListingsTemplate {
    listings: Vec<Listing>,
}

async fn listings() -> ListingsTemplate {
    info!("Fetching schedule data...");
    let listings = fetch_listings().await.unwrap_or_default();
    ListingsTemplate { listings }
}
