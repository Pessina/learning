use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug)]
pub struct RedisCell {
    pub value: String,
    pub expiry: Option<DateTime<Utc>>,
}

pub struct Redis {
    map: HashMap<String, RedisCell>,
}

impl Redis {
    pub fn new() -> Self {
        Redis {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: RedisCell) -> Option<RedisCell> {
        self.map.insert(key, value)
    }

    pub fn get(&mut self, key: &str) -> Option<&RedisCell> {
        if let Some(expiry) = self.map.get(key).and_then(|cell| cell.expiry) {
            if expiry > Utc::now() {
                self.map.get(key)
            } else {
                self.delete(key);
                None
            }
        } else {
            self.map.get(key)
        }
    }

    pub fn delete(&mut self, key: &str) -> Option<RedisCell> {
        self.map.remove(key)
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn it_should_succeed_get() {
        let mut redis = Redis::new();

        let key = "Name";
        let value = RedisCell {
            value: String::from("Felipe"),
            expiry: None,
        };

        redis.set(key.to_string(), value);
        let result = redis.get(key).unwrap();
        assert_eq!(result.value, "Felipe".to_string());
    }

    #[test]
    fn it_should_fail_get() {
        let mut redis = Redis::new();

        let key_set = "Name";
        let key_get = "Age";
        let value = RedisCell {
            value: String::from("Felipe"),
            expiry: None,
        };

        redis.set(key_set.to_string(), value);
        match redis.get(key_get) {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn it_should_overwrite_insert() {
        let mut redis = Redis::new();

        let key = "Name";
        let value = RedisCell {
            value: String::from("Felipe"),
            expiry: None,
        };

        redis.set(key.to_string(), value);
        let value = RedisCell {
            value: String::from("Carlos"),
            expiry: None,
        };

        redis.set(key.to_string(), value);

        if let Some(result) = redis.get(key) {
            assert_eq!(result.value, "Carlos".to_string());
        }
    }

    #[test]
    fn it_should_be_expired() {
        let mut redis = Redis::new();

        let key = "Name";
        let value = RedisCell {
            value: String::from("Carlos"),
            expiry: Some(Utc::now() - Duration::seconds(10)),
        };

        redis.set(key.to_string(), value);

        match redis.get(key) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn it_should_not_be_expired() {
        let mut redis = Redis::new();

        let key = "Name";
        let value = RedisCell {
            value: String::from("Carlos"),
            expiry: Some(Utc::now() + Duration::hours(1)),
        };

        redis.set(key.to_string(), value);

        match redis.get(key) {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }
}
