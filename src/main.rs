use askama::Template;
use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

use dagskra::{fetch_listings, Listings};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    date: String,
    listings: Listings,
    title: &'a str,
}

async fn index() -> IndexTemplate<'static> {
    let listings = fetch_listings().await.unwrap_or_default();
    let date = listings.first().map_or("".to_owned(), |l| l.date());
    IndexTemplate {
        date,
        listings,
        title: "Dagskrá RÚV",
    }
}
