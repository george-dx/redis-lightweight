use std::env::args;

pub struct Config {
    dir: Option<String>,
    dbfilename: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            dir: None,
            dbfilename: None,
        }
    }

    pub fn set(&mut self) {
        let args = args().collect::<Vec<_>>();
        if args.contains(&String::from("--dir")) {
            self.dir = args
                .get(args.iter().position(|n| n == "--dir").unwrap() + 1)
                .cloned();
        }
        if args.contains(&String::from("--dbfilename")) {
            self.dbfilename = args
                .get(args.iter().position(|n| n == "--dbfilename").unwrap() + 1)
                .cloned();
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key.to_lowercase().as_str() {
            "dir" => self.dir.clone(),
            "dbfilename" => self.dbfilename.clone(),
            _ => None,
        }
    }
}
