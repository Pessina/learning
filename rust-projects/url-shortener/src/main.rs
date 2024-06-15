use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::net::TcpListener;
use url_shortener::modules::routes::{echo, ping, save};

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = Arc::new(Mutex::new(client.get_connection().unwrap()));

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/echo", get(echo))
        .route("/save", post(save))
        .layer(Extension(con));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
