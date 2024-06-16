use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::net::TcpListener;
use url_shortener::modules::{
    routes::{add_url, get_all, redirect},
    store::Store,
};

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(Store::new()));

    let app = Router::new()
        .route("/add_url", post(add_url))
        .route("/get_all", get(get_all))
        .route("/:url_hash", get(redirect))
        .layer(Extension(store));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
