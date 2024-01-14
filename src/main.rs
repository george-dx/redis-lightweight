use std::io::{prelude::*, Error};
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;
const PING: &str = "*1\r\n$4\r\nping\r\n";
const PONG: &str = "+PONG\r\n";

fn respond_with_pong(stream: &mut TcpStream) {
    stream.write(PONG.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(mut _stream) => {
            println!("Accepted new connection");
            loop {
                let mut buffer = [0; BUFFER_SIZE];
                let size = _stream.read(&mut buffer).unwrap();
                if size == 0 {
                    break;
                }
                let command = String::from_utf8_lossy(&buffer[..size]);

                match command.as_ref() {
                    PING => {
                        respond_with_pong(&mut _stream);
                    }
                    _ => {
                        println!("Unknown command: {:?}", command);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error at stream incoming: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind the listener");

    for stream in listener.incoming() {
        handle_connection(stream);
    }
}
