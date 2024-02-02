use askama::Template;
use axum::{routing, Router};
use axum_htmx::HxRequestGuardLayer;
use chrono::Utc;
use shuttle_axum::ShuttleAxum;
use tracing::info;

use dagskra::{fetch_listings, Listing};

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    info!("Starting axum app...");
    let htmx_router = Router::new()
        .route("/_listings", routing::get(listings))
        .layer(HxRequestGuardLayer::default());
    let router = Router::new()
        .merge(htmx_router)
        .route("/", routing::get(index));
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
#[template(path = "_listings.html")]
struct ListingsTemplate {
    listings: Vec<Listing>,
}

async fn listings() -> ListingsTemplate {
    let listings = fetch_listings().await.unwrap_or_default();
    ListingsTemplate { listings }
}
