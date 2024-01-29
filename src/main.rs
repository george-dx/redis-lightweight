mod database;

use database::database::Database;
use std::io::{prelude::*, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;

const BUFFER_SIZE: usize = 1024;
const PING: &str = "*1\r\n$4\r\nping\r\n";
const PONG: &str = "+PONG\r\n";
const ECHO: &str = "*2\r\n$4\r\necho\r\n";
const SET: &str = "$3\r\nset\r\n";
const GET: &str = "$3\r\nget\r\n";
const OK: &str = "+OK\r\n";

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

fn database_set(database: &mut Database, stream: &mut TcpStream, command: &str) {
    let dollar = "$";
    let splitted_command = command.split(dollar).collect::<Vec<&str>>();
    database.set(splitted_command[2], splitted_command[3]);
    let _ = stream.write(OK.as_bytes());
}

fn database_get(database: &mut Database, stream: &mut TcpStream, command: &str) {
    let dollar = "$";
    let splitted_command = command.split(dollar).collect::<Vec<&str>>();
    if let Some(value) = database.get(splitted_command[2]) {
        let response_parts = value.split("\r\n").collect::<Vec<&str>>();
        let response = response_parts[1];
        let _ = stream.write(format!("+{response}\r\n").as_bytes());
    } else {
        let _ = stream.write("+\r\n".as_bytes());
    };
}

fn handle_connection(stream: Result<TcpStream, Error>) {
    let mut db = Database::new();
    match stream {
        Ok(mut _stream) => loop {
            let mut buffer = [0; BUFFER_SIZE];
            let size = _stream.read(&mut buffer).unwrap();
            if size == 0 {
                break;
            }
            let command = String::from_utf8_lossy(&buffer[..size]).to_string();
            let command_str = command.as_str();

            if command_str.contains(PING) {
                respond_with_pong(&mut _stream);
            } else if command_str.contains(ECHO) {
                respond_with_message(&mut _stream, command_str)
            } else if command_str.contains(SET) {
                database_set(&mut db, &mut _stream, command_str);
            } else if command_str.contains(GET) {
                database_get(&mut db, &mut _stream, command_str);
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
