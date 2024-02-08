use std::{io::Write, net::TcpStream, time::Duration};

use crate::{database::database::Database, OK};

pub struct DatabaseInteractor {
    database: Database,
}

impl DatabaseInteractor {
    pub fn new(database: Database) -> DatabaseInteractor {
        DatabaseInteractor { database: database }
    }

    pub fn database_set(&mut self, stream: &mut TcpStream, command: &str) {
        let dollar = "$";
        let splitted_command = command.split(dollar).collect::<Vec<&str>>();
        if let Some(value) = splitted_command.get(5) {
            let expiry_in_ms = &value[1..].replace("\r\n", "");
            match expiry_in_ms.parse::<u64>() {
                Ok(conversion) => {
                    let expiry_duration = Duration::from_millis(conversion);
                    self.database.set(
                        splitted_command[2].to_string(),
                        splitted_command[3].to_string(),
                        Some(expiry_duration),
                    );
                    let _ = stream.write(OK.as_bytes());
                }
                Err(e) => {
                    println!("Error at u64 parse: {}", e);
                    let _ = stream.write("+\r\n".as_bytes());
                }
            }
        } else {
            self.database.set(
                splitted_command[2].to_string(),
                splitted_command[3].to_string(),
                None,
            );
            let _ = stream.write(OK.as_bytes());
        }
    }

    pub fn database_get(&self, stream: &mut TcpStream, command: &str) {
        let dollar = "$";
        let splitted_command = command.split(dollar).collect::<Vec<&str>>();
        if let Some(value) = self.database.get(splitted_command[2].to_string()) {
            let response_parts = value.split("\r\n").collect::<Vec<&str>>();
            let response = response_parts[1];
            let _ = stream.write(format!("+{response}\r\n").as_bytes());
        } else {
            let _ = stream.write("$-1\r\n".as_bytes());
        };
    }

    pub fn database_get_keys(&self) -> Option<Vec<String>> {
        let mut keys: Vec<String> = vec![];
        for (key, _) in self.database.get_keys() {
            println!("@@@@@@{}", key);
            keys.push(key);
        }
        if keys.is_empty() {
            return None;
        }
        return Some(keys);
    }
}
