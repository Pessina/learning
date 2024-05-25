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
