use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Extension, Json, Path, Query},
    response::Redirect,
};
use serde::Deserialize;

use super::store::{Store, UrlMap};

pub async fn ping() -> String {
    "pong".to_string()
}

pub async fn echo(Query(params): Query<HashMap<String, String>>) -> String {
    format!("{}", params.get("echo").unwrap())
}

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

pub async fn add_url(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Json(payload): Json<AddUrlRequest>,
) -> String {
    store.lock().unwrap().add(payload.url.as_str())
}

#[derive(Deserialize)]
pub struct GetUrlRequest {
    url: String,
}

pub async fn get_url(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Json(payload): Json<GetUrlRequest>,
) -> Json<UrlMap> {
    Json(store.lock().unwrap().get(payload.url.as_str()))
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
    let url_map = store.lock().unwrap().get(&path.url_hash);
    println!("{:?}", url_map);
    Redirect::permanent(&url_map.original)
}
