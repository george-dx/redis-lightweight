use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

fn respond_with_pong(stream: &mut TcpStream) {
    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind the listener");

    for stream in listener.incoming() {
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
                        "*1\r\n$4\r\nping\r\n" => {
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
}
