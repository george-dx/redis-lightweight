use std::collections::HashMap;

pub struct Database {
    db: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database { db: HashMap::new() }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.db.get(key)
    }

    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.db.insert(key.to_owned(), value.to_owned())
    }
}

fn respond_with_pong(stream: &mut TcpStream) {
    stream.write(PONG.as_bytes()).unwrap();
    stream.flush().unwrap();
}
