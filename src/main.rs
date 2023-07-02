#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting app");
    let router = axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/_listings", axum::routing::get(listings));
    Ok(router.into())
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
}

async fn index() -> IndexTemplate {
    let title = "Dagskrá RÚV";
    IndexTemplate { title }
}

#[derive(askama::Template)]
#[template(path = "_listings.html")]
struct ListingsTemplate {
    date: String,
    listings: dagskra::Listings,
}

async fn listings() -> ListingsTemplate {
    tracing::info!("Fetching schedule data");
    let listings = dagskra::fetch_listings().await.unwrap_or_default();
    let date = listings
        .first()
        .map_or_else(|| "".to_string(), |listing| listing.date());
    ListingsTemplate { date, listings }
}
