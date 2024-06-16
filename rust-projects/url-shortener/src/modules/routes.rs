use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Json, Path},
    response::Redirect,
};
use serde::Deserialize;

use super::store::Store;

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

pub async fn add_url(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Json(payload): Json<AddUrlRequest>,
) -> String {
    store.lock().unwrap().add(payload.url)
}

#[derive(Deserialize)]
pub struct RedirectRequest {
    url_hash: String,
}

pub async fn redirect(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Path(path): Path<RedirectRequest>,
) -> Redirect {
    println!("{:?}", path.url_hash);
    let url_map = store.lock().unwrap().get(path.url_hash);
    println!("{:?}", url_map);
    Redirect::permanent(&url_map.original)
}
