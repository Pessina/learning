use std::collections::HashMap;

use chrono::prelude::*;

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

    pub fn get(&self, key: &str) -> Option<&RedisCell> {
        self.map.get(key)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
}
