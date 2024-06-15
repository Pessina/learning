use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::net::TcpListener;
use url_shortener::modules::{
    routes::{add_url, echo, get_url, ping},
    store::Store,
};

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(Store::new()));

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/echo", get(echo))
        .route("/add_url", post(add_url))
        .route("/get_url", post(get_url))
        .layer(Extension(store));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
