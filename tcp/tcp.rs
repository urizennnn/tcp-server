use crate::threadpool::thread::Threadpool;

use super::{handle_receiver::handle_receiver, handle_sender::handle_sender};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::exit,
};

pub struct TCP {}

impl TCP {
    pub fn run(addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Server listening on {}", addr);

        let pool = match Threadpool::build(6) {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("Failed to build thread pool: {}", e);
                exit(1);
            }
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    pool.execute(move || {
                        TCP::handle_client(stream);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        println!("Server shutdown.");
    }

    pub fn handle_client(mut stream: TcpStream) {
        loop {
            let mut buffer = [0; 1024];

            // Read from the TCP stream
            let input = match stream.read(&mut buffer) {
                Ok(size) => size,
                Err(e) => {
                    eprintln!("Error reading from stream: {}", e);
                    break;
                }
            };

            if input == 0 {
                println!("Connection closed by the client");
                break;
            }

            let request = String::from_utf8_lossy(&buffer[..input]);

            if request.starts_with("GET") {
                println!("GET function triggered");
                let content = fs::read_to_string(Path::new("render/base.html")).unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                );
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Error writing response: {}", e);
                    break;
                }
                if let Err(e) = stream.flush() {
                    eprintln!("Error flushing stream: {}", e);
                    break;
                }
            } else if request.starts_with("POST /receiver") {
                println!("Receiver function triggered");
                if let Err(e) = handle_receiver(&mut stream, &buffer[..input]) {
                    eprintln!("Error handling receiver: {}", e);
                }
            } else if request.starts_with("POST /sender") {
                println!("Sender function triggered");
                match handle_sender(&buffer) {
                    Ok(response) => {
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            eprintln!("Error writing sender response: {}", e);
                        }
                        if let Err(e) = stream.flush() {
                            eprintln!("Error flushing sender response: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error handling sender: {}", e);
                        let error_response = format!(
                "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\n\r\nInternal Server Error",
                21
            );
                        let _ = stream.write_all(error_response.as_bytes());
                        let _ = stream.flush();
                    }
                }
            } else {
                println!("Unknown request: {}", request);
            }
        }
    }
}
