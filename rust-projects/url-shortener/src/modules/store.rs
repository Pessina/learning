use redis::{Commands, Connection, Iter, RedisError};
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

    pub fn add(&mut self, url: String) -> Result<String, RedisError> {
        let url_hash = format!("{:x}", xxh3_64(url.as_bytes()));
        self.map
            .set::<String, String, String>(url_hash.to_string(), url)?;
        Ok(url_hash)
    }

    pub fn get(&mut self, url_hash: String) -> Result<UrlMap, RedisError> {
        let url = self.map.get::<String, String>(url_hash.clone())?;
        Ok(UrlMap {
            hash: url_hash.to_string(),
            original: url.to_string(),
            short: format!("{}/{}", SERVER_URL, url_hash),
        })
    }

    pub fn get_all(&mut self) -> Result<Vec<UrlMap>, RedisError> {
        let iter: Iter<String> = self.map.scan()?;
        let mut keys = Vec::new();
        iter.for_each(|key| keys.push(key));

        let mut ret = Vec::new();
        for url_hash in keys {
            let value = self.get(url_hash.to_string())?;
            ret.push(value)
        }
        Ok(ret)
    }

    pub fn delete(&mut self, url: &str) -> Result<String, RedisError> {
        let url_hash = format!("{:x}", xxh3_64(url.as_bytes()));
        self.map.del(url_hash)?;
        Ok(url.to_string())
    }
}
