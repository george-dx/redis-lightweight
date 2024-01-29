use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database { db: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.db.insert(key.to_owned(), value.to_owned())
    }
}
