use askama::Template;
use axum::{routing::get, serve, Router};
use chrono::Utc;
use tokio::net::TcpListener;

use dagskra::{fetch_listings, Listing};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/_listings", get(listings));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    date: String,
    title: &'static str,
}

async fn index() -> IndexTemplate {
    let title = "Dagskrá RÚV";
    let date = Utc::now().format("%d.%m.%Y").to_string();
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
