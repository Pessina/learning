use std::collections::HashMap;

pub struct Redis {
    map: HashMap<String, String>,
}

impl Redis {
    pub fn new() -> Self {
        Redis {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_should_succeed_get() {
        let mut redis = Redis::new();
        redis.set("Name".to_string(), "Felipe".to_string());
        let name = redis.get("Name").unwrap().to_owned();
        assert_eq!(name, "Felipe".to_string());
    }

    #[test]
    fn it_should_fail_get() {
        let mut redis = Redis::new();
        redis.set("Name".to_string(), "Felipe".to_string());
        match redis.get("Age") {
            None => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn it_should_overwrite_insert() {
        let mut redis = Redis::new();
        redis.set("Name".to_string(), "Felipe".to_string());
        redis.set("Name".to_string(), "Carlos".to_string());
        let name = redis.get("Name").unwrap().to_owned();
        assert_eq!(name, "Carlos".to_string());
    }
}
