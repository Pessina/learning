use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::extract::{Extension, Json, Query};
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
