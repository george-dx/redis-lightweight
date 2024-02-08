mod config;
mod database;
mod database_interactor;

use config::config::Config;
use database::database::Database;
use database_interactor::database_interactor::DatabaseInteractor;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

const BUFFER_SIZE: usize = 1024;
const PING: &str = "*1\r\n$4\r\nping\r\n";
const PONG: &str = "+PONG\r\n";
const ECHO: &str = "*2\r\n$4\r\necho\r\n";
const SET: &str = "$3\r\nset\r\n";
const GET: &str = "$3\r\nget\r\n";
const CONFIG: &str = "$6\r\nconfig\r\n$";
const OK: &str = "+OK\r\n";

fn _find_key_in_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer: [u8; 32] = [0; 32];
    reader.read_exact(&mut buffer)?;

    let keys = std::str::from_utf8(&buffer)?.to_string();

    Ok(keys)
}

fn to_bulk_string(get_type: &str, message: &str) -> String {
    let message_len = message.len();
    let type_len = get_type.len();
    format!(
        "*2\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
        type_len, get_type, message_len, message
    )
}

fn respond_with_message(stream: &mut TcpStream, command: &str) {
    let dollar = "$";
    let splitted_command = command.split(dollar).collect::<Vec<&str>>();
    let response = dollar.to_string() + splitted_command[2];
    let _ = stream.write(response.as_bytes());
}

fn respond_with_pong(stream: &mut TcpStream) {
    stream.write(PONG.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(stream: Result<TcpStream, Error>) {
    let db = Database::new();
    let mut db_interactor = DatabaseInteractor::new(db);
    let mut config = Config::new();
    config.set();
    match stream {
        Ok(mut _stream) => loop {
            let mut buffer = [0; BUFFER_SIZE];
            let size = _stream.read(&mut buffer).unwrap();
            if size == 0 {
                break;
            }
            let command = String::from_utf8_lossy(&buffer[..size]).to_string();
            let command_str = command.as_str();
            println!("{:?}", command_str);
            if command_str.contains(PING) {
                respond_with_pong(&mut _stream);
            } else if command_str.contains(ECHO) {
                respond_with_message(&mut _stream, command_str)
            } else if command_str.contains(SET) {
                db_interactor.database_set(&mut _stream, command_str);
            } else if command_str.contains(GET) && !command_str.contains(CONFIG) {
                db_interactor.database_get(&mut _stream, command_str);
            } else if command_str.contains(CONFIG) {
                let message_type;
                if command_str.contains("dir") {
                    message_type = "dir";
                } else {
                    message_type = "dbfilename";
                }
                if let Some(message) = config.get(message_type) {
                    let _ = _stream.write_all(to_bulk_string(message_type, &message).as_bytes());
                }
            } else if command_str.contains("keys") {
                if let Some(dir) = config.get("dir") {
                    if let Some(db_filename) = config.get("dbfilename") {
                        let keys = _find_key_in_file(Path::new(&(dir+"/"+&db_filename)));
                        println!("YYYYYYYYY {:?}", keys);
                        // println!("{}",db_interactor.database_get(stream, command))
                        if let Some(keys) = db_interactor.database_get_keys() {
                            println!("@@@@@{:?}", keys);
                            let _ = _stream.write_all(
                                format!("*1\r\n${}\r\n{:?}\r\n", keys.len(), keys).as_bytes(),
                            );
                        } else {
                            println!("No keys found in db file");
                            let _ = _stream.write_all("$-1\r\n".as_bytes());
                        };
                    }
                }
            } else {
                println!("Unknown command: {:?}", command);
            }
        },
        Err(e) => {
            println!("Error at stream incoming: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind the listener");

    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}
