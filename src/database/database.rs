use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

pub struct Database {
    db: HashMap<String, (String, Option<SystemTime>)>,
}

impl Database {
    pub fn new() -> Database {
        Database { db: HashMap::new() }
    }

    pub fn get(&self, key: String) -> Option<&String> {
        if let Some((value, exp_time)) = self.db.get(&key) {
            if exp_time.is_none() || (exp_time.unwrap() > SystemTime::now()) {
                return Some(value);
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, key: String, value: String, expire_ms: Option<Duration>) -> () {
        if let Some(expire_ms) = expire_ms {
            // println!("{:?}", SystemTime::now() + expire_ms);
            self.db
                .insert(key, (value, Some(SystemTime::now() + expire_ms)));
        } else {
            self.db.insert(key, (value, None));
        };
    }
}
