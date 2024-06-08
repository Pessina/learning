use std::io::Read;
use std::{collections::HashMap, io::Write};

use std::fs::{self, File};
use std::path::Path;

use chrono::serde::ts_seconds_option;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::string_array::{insert_on_array, ArrayPlacement};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisCell {
    pub value: String,
    #[serde(with = "ts_seconds_option")]
    pub expiry: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redis {
    map: HashMap<String, RedisCell>,
}

const REDIS_STORE_DIR: &str = "redis_store";

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

    pub fn set_list(
        &mut self,
        key: String,
        value: String,
        placement: ArrayPlacement,
    ) -> Result<u32, String> {
        let redis_return = self.get(&key);
        match redis_return {
            Some(cell) => {
                let (new_value, len) = insert_on_array(cell.value.as_str(), &value, placement)?;
                let expiry = cell.expiry;
                self.set(
                    key,
                    RedisCell {
                        value: new_value,
                        expiry,
                    },
                );

                Ok(len)
            }
            None => {
                self.set(
                    key.to_string(),
                    RedisCell {
                        value: format!("[{}]", value),
                        expiry: None,
                    },
                );

                Ok(1)
            }
        }
    }

    pub fn save(self, name: &str) {
        let serialized = serde_json::to_string(&self).unwrap();

        fs::create_dir_all(REDIS_STORE_DIR).expect("Failed to crete directory");

        let path = format!("{}/{}", REDIS_STORE_DIR, name);
        let path = Path::new(&path);

        let mut file = File::create(path).expect("Failed opening file");
        file.write(serialized.as_bytes())
            .expect("Error writing to file");
    }

    pub fn load(name: &str) -> Redis {
        let path = format!("{}/{}", REDIS_STORE_DIR, name);
        let path = Path::new(&path);

        let mut file = File::open(path).expect("Error opening file");

        let mut serialized = String::new();
        file.read_to_string(&mut serialized).unwrap();

        serde_json::from_str::<Redis>(&serialized).unwrap()
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

    #[test]
    fn should_create_and_insert_on_array() {
        let mut redis = Redis::new();

        let result = redis.set_list(
            "arr".to_string(),
            "first".to_string(),
            ArrayPlacement::RIGHT,
        );

        assert_eq!(1, result.unwrap());
        assert_eq!("[first]", redis.get("arr").unwrap().value);

        let result = redis.set_list(
            "arr".to_string(),
            "second".to_string(),
            ArrayPlacement::RIGHT,
        );

        assert_eq!(2, result.unwrap());
        assert_eq!("[first,second]", redis.get("arr").unwrap().value);

        let result = redis.set_list("arr".to_string(), "third".to_string(), ArrayPlacement::LEFT);

        assert_eq!(3, result.unwrap());
        assert_eq!("[third,first,second]", redis.get("arr").unwrap().value)
    }

    #[test]
    fn should_fail_fail_insert_on_array_string() {
        let mut redis = Redis::new();

        redis.set(
            "arr".to_string(),
            RedisCell {
                value: "first".to_string(),
                expiry: None,
            },
        );

        let result = redis.set_list("arr".to_string(), "third".to_string(), ArrayPlacement::LEFT);

        assert!(result.is_err())
    }

    #[test]
    fn should_serialize_and_deserialize() {
        let mut redis = Redis::new();

        redis.set(
            "first".to_string(),
            RedisCell {
                value: "1".to_string(),
                expiry: None,
            },
        );

        redis.set(
            "second".to_string(),
            RedisCell {
                value: "2".to_string(),
                expiry: Some(DateTime::from(
                    Utc.timestamp_opt((10 as i64).pow(10), 0).unwrap(),
                )),
            },
        );

        redis.set(
            "third".to_string(),
            RedisCell {
                value: "3".to_string(),
                expiry: None,
            },
        );

        redis.set(
            "fourth".to_string(),
            RedisCell {
                value: "4".to_string(),
                expiry: Some(DateTime::from(Utc.timestamp_opt(1_000_000, 0).unwrap())),
            },
        );

        let serialized = serde_json::to_string(&redis).unwrap();
        let redis_deserialized: Redis = serde_json::from_str(&serialized).unwrap();

        let first = redis_deserialized.map.get("first").unwrap();
        assert_eq!("1", first.value);
        assert_eq!(None, first.expiry);

        let second = redis_deserialized.map.get("second").unwrap();
        assert_eq!("2", second.value);
        assert_eq!(
            Some(DateTime::from(
                Utc.timestamp_opt((10 as i64).pow(10), 0).unwrap(),
            )),
            second.expiry
        );

        let third = redis_deserialized.map.get("third").unwrap();
        assert_eq!("3", third.value);
        assert_eq!(None, third.expiry);

        let fourth = redis_deserialized.map.get("fourth").unwrap();
        assert_eq!("4", fourth.value);
        assert_eq!(
            Some(DateTime::from(Utc.timestamp_opt(1_000_000, 0).unwrap())),
            fourth.expiry
        );

        assert_eq!(redis_deserialized.map.len(), 4);
    }

    #[test]
    fn should_serialize_and_deserialize_empty() {
        let redis = Redis::new();

        let serialized = serde_json::to_string(&redis).unwrap();
        let redis_deserialized: Redis = serde_json::from_str(&serialized).unwrap();

        println!("{}", redis_deserialized.map.len());

        assert_eq!(redis_deserialized.map.len(), 0);
    }

    #[test]
    fn should_save_and_load() {
        let mut redis = Redis::new();

        redis.set(
            "Name".to_string(),
            RedisCell {
                value: "Felipe".to_string(),
                expiry: None,
            },
        );

        redis.set(
            "BirthDate".to_string(),
            RedisCell {
                value: "02/09/1980".to_string(),
                expiry: Some(Utc.timestamp_opt(100_000_000_000, 0).unwrap()),
            },
        );

        redis
            .set_list(
                "Friends".to_string(),
                "Marcos".to_string(),
                ArrayPlacement::LEFT,
            )
            .unwrap();

        redis
            .set_list(
                "Friends".to_string(),
                "Carlos".to_string(),
                ArrayPlacement::LEFT,
            )
            .unwrap();

        redis
            .set_list(
                "Friends".to_string(),
                "Marcelo".to_string(),
                ArrayPlacement::LEFT,
            )
            .unwrap();

        redis.save("redis.txt");
        let mut redis = Redis::load("redis.txt");

        let name = redis.get("Name").unwrap();
        assert_eq!("Felipe", name.value);
        assert_eq!(None, name.expiry);

        let birth_date = redis.get("BirthDate").unwrap();
        assert_eq!("02/09/1980", birth_date.value);
        assert_eq!(
            Some(Utc.timestamp_opt(100_000_000_000, 0).unwrap()),
            birth_date.expiry
        );

        let friends = redis.get("Friends").unwrap();
        assert_eq!("[Marcelo,Carlos,Marcos]", friends.value);

        redis.delete("Friends");

        redis.save("redis.txt");
        let mut redis = Redis::load("redis.txt");

        let friends = redis.get("Friends");
        assert!(friends.is_none())
    }
}
