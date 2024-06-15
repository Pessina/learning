use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::extract::{Extension, Json, Query};
use redis::{Commands, Connection};
use serde::Deserialize;

pub async fn ping() -> String {
    "pong".to_string()
}

pub async fn echo(Query(params): Query<HashMap<String, String>>) -> String {
    format!("{}", params.get("echo").unwrap())
}

#[derive(Deserialize)]
pub struct KeyValue {
    key: String,
    value: String,
}

pub async fn save(
    Extension(con): Extension<Arc<Mutex<Connection>>>,
    Json(payload): Json<KeyValue>,
) -> String {
    con.lock()
        .unwrap()
        .set::<String, String, ()>(payload.key.to_string(), payload.value)
        .unwrap();

    con.lock().unwrap().get(payload.key).unwrap()
}
