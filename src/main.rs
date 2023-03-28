use askama::Template;
use axum::{routing, Router, Server};
use dotenvy::dotenv;
use std::net::SocketAddr;

use dagskra::{fetch_listings, Listings};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/", routing::get(index))
        .route("/_listings", routing::get(listings));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
    let listings = fetch_listings().await.unwrap_or_default();
    let date = listings.first().map_or("".to_string(), |l| l.date());
    ListingsTemplate { date, listings }
}
