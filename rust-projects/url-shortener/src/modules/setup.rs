use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};

use super::routes::{add_url, delete_url, get_all, redirect};
use super::store::Store;

pub async fn setup_server() -> Router {
    let store = Arc::new(Mutex::new(Store::new()));

    Router::new()
        .route("/add_url", post(add_url))
        .route("/get_all", get(get_all))
        .route("/:url_hash", get(redirect))
        .route("/delete_url", delete(delete_url))
        .layer(Extension(store))
}
