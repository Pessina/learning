use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};

use super::store::Store;

#[derive(Deserialize)]
pub struct AddUrlRequest {
    url: String,
}

#[derive(Serialize)]
struct AddUrlResponse {
    hashed_url: String,
}

pub async fn add_url(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Json(payload): Json<AddUrlRequest>,
) -> impl IntoResponse {
    match store.lock() {
        Ok(mut store) => match store.add(payload.url) {
            Ok(hashed_url) => {
                let response = AddUrlResponse { hashed_url };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RedirectRequest {
    url_hash: String,
}

pub async fn redirect(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Path(path): Path<RedirectRequest>,
) -> impl IntoResponse {
    match store.lock() {
        Ok(mut store) => match store.get(path.url_hash) {
            Ok(url_map) => Redirect::permanent(&url_map.original).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
