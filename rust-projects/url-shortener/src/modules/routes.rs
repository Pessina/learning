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

#[derive(Serialize, Deserialize)]
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
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "URL not found").into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_all(Extension(store): Extension<Arc<Mutex<Store>>>) -> impl IntoResponse {
    match store.lock() {
        Ok(mut store) => match store.get_all() {
            Ok(res) => Json(res).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    url: String,
}

pub async fn delete_url(
    Extension(store): Extension<Arc<Mutex<Store>>>,
    Json(payload): Json<DeleteRequest>,
) -> impl IntoResponse {
    match store.lock() {
        Ok(mut store) => match store.delete(&payload.url) {
            Ok(res) => Json(res).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{to_bytes, Body},
        extract::Request,
    };

    use hyper::header::LOCATION;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::modules::setup::setup_server;

    use super::*;

    #[tokio::test]
    async fn test_add_url() {
        let app = setup_server().await;

        let base_url = "https://example.com";

        let payload = json!({ "url": base_url });
        let request = Request::builder()
            .uri("/add_url")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_data: AddUrlResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_data.hashed_url, "9398cc7c078760e6");

        let request = Request::builder()
            .uri(format!("/{}", response_data.hashed_url))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(response.headers().get(LOCATION).unwrap(), &base_url);
    }
}
