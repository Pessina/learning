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

use axum::{
    routing::{delete, get, post},
    Router,
};

pub async fn setup_router() -> Router {
    let store = Arc::new(Mutex::new(Store::new()));

    Router::new()
        .route("/add_url", post(add_url))
        .route("/get_all", get(get_all))
        .route("/:url_hash", get(redirect))
        .route("/delete_url", delete(delete_url))
        .layer(Extension(store))
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
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
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
            Ok(res) => (StatusCode::OK, Json(res)).into_response(),
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

    use crate::modules::store::UrlMap;

    use super::*;

    #[tokio::test]
    async fn test_add_and_redirect() {
        let original_url = "https://example.com";
        let app = setup_router().await;

        let request = Request::builder()
            .uri("/add_url")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(json!({ "url": original_url }).to_string()))
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
        assert_eq!(response.headers().get(LOCATION).unwrap(), &original_url);
    }

    #[tokio::test]
    async fn test_get_all_and_delete() {
        let original_url = "https://example.com";
        let app = setup_router().await;

        let request = Request::builder()
            .uri("/add_url")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(json!({"url": original_url}).to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder()
            .uri("/get_all")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_data: Vec<UrlMap> = serde_json::from_slice(&body).unwrap();

        println!("{:?}", response_data);

        assert!(response_data.len() > 0);
        assert!(response_data.contains(&UrlMap {
            hash: "9398cc7c078760e6".to_string(),
            original: original_url.to_string(),
            short: "http://localhost:3000/9398cc7c078760e6".to_string()
        }));

        let request = Request::builder()
            .uri("/delete_url")
            .method("DELETE")
            .header("content-type", "application/json")
            .body(Body::from(json!({"url": original_url}).to_string()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_data: String = serde_json::from_slice(&body).unwrap();

        assert_eq!(response_data, original_url);

        let request = Request::builder()
            .uri("/get_all")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_data: Vec<UrlMap> = serde_json::from_slice(&body).unwrap();

        println!("{:?}", response_data);

        assert!(response_data.len() > 0);
        assert!(!response_data.contains(&UrlMap {
            hash: "9398cc7c078760e6".to_string(),
            original: original_url.to_string(),
            short: "http://localhost:3000/9398cc7c078760e6".to_string()
        }));
    }
}
