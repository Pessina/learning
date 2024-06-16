use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

const SERVER_URL: &str = "http://localhost:3000";

pub struct Store {
    map: Connection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlMap {
    pub hash: String,
    pub original: String,
    pub short: String,
}

impl Store {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let con = client.get_connection().unwrap();

        Store { map: con }
    }

    pub fn add(&mut self, url: String) -> String {
        let url_hash = format!("{:x}", xxh3_64(url.as_bytes()));
        self.map
            .set::<String, String, String>(url_hash.to_string(), url)
            .expect("Failed to add url");
        url_hash
    }

    pub fn get(&mut self, url_hash: String) -> UrlMap {
        let url = self.map.get::<String, String>(url_hash.clone()).unwrap();
        UrlMap {
            hash: url_hash.to_string(),
            original: url.to_string(),
            short: format!("{}/{}", SERVER_URL, url_hash),
        }
    }
}
