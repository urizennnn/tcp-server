use crate::{
    tcp::{handle_receiver::handle_receiver, handle_sender::handle_sender},
    threadpool::thread::Threadpool,
};
use regex::Regex;
use serde::Deserialize;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

use std::io;

#[derive(Deserialize, Debug)]
pub struct TCP {}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub data: String,
}

impl TCP {
    pub fn conver_to_json(buffer: &[u8]) -> Result<Data, io::Error> {
        let request = String::from_utf8_lossy(buffer);

        let re = Regex::new(r"\r\n\r\n(.*)").unwrap();
        let json_payload = if let Some(caps) = re.captures(&request) {
            caps.get(1).map_or("", |m| m.as_str())
        } else {
            ""
        };

        let json: Data = match serde_json::from_str(json_payload) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON"));
            }
        };

        Ok(json)
    }

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

        let content = Arc::new(Mutex::new(
            fs::read_to_string(Path::new("render/base/base.html")).unwrap(),
        ));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let content = Arc::clone(&content);
                    pool.execute(move || {
                        TCP::handle_client(stream, content);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        eprintln!("Server shutdown.");
    }

    pub fn handle_client(mut stream: TcpStream, content: Arc<Mutex<String>>) {
        let mut buffer = [0; 1024];

        loop {
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
            let request_line = request.lines().next().unwrap_or("");

            if request_line.starts_with("GET") {
                TCP::handle_get(&mut stream, &content);
            } else if request_line.starts_with("POST /update") {
                TCP::handle_post_update(&mut stream, &buffer[..input], &content);
            } else if request_line.starts_with("POST /receiver") {
                TCP::handle_post_receiver(&mut stream, &buffer[..input]);
            } else if request_line.starts_with("POST /sender") {
                TCP::handle_post_sender(&mut stream, &buffer[..input]);
            } else {
                println!("Unknown request: {}", request_line);
                TCP::handle_unknown(&mut stream);
            }
        }
    }

    fn handle_get(stream: &mut TcpStream, content: &Arc<Mutex<String>>) {
        println!("GET function triggered");

        let response_body = {
            let content = content.lock().unwrap();
            content.clone()
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error writing response: {}", e);
        }
        if let Err(e) = stream.flush() {
            eprintln!("Error flushing stream: {}", e);
        }
    }

    fn handle_post_update(stream: &mut TcpStream, buffer: &[u8], content: &Arc<Mutex<String>>) {
        println!("Update function triggered");

        match TCP::conver_to_json(buffer) {
            Ok(json) => {
                let mut content = content.lock().unwrap();
                *content = json.data;

                let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Error writing response: {}", e);
                }
                if let Err(e) = stream.flush() {
                    eprintln!("Error flushing stream: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                TCP::send_internal_error(stream);
            }
        }
    }

    fn handle_post_receiver(stream: &mut TcpStream, buffer: &[u8]) {
        println!("Receiver function triggered");
        match handle_receiver(stream, buffer) {
            Ok(_) => println!("Receiver handled successfully."),
            Err(e) => {
                eprintln!("Error handling receiver: {}", e);
                TCP::send_internal_error(stream);
            }
        }
    }

    fn handle_post_sender(stream: &mut TcpStream, _buffer: &[u8]) {
        println!("Sender function triggered");
        match handle_sender(stream) {
            Ok(_) => println!("Sender handled successfully."),
            Err(e) => {
                eprintln!("Error handling sender: {}", e);
                TCP::send_internal_error(stream);
            }
        }
    }

    fn handle_unknown(stream: &mut TcpStream) {
        let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error writing unknown request response: {}", e);
        }
    }

    fn send_internal_error(stream: &mut TcpStream) {
        let error_response =
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\n\r\nInternal Server Error";
        if let Err(e) = stream.write_all(error_response.as_bytes()) {
            eprintln!("Error writing internal error response: {}", e);
        }
        if let Err(e) = stream.flush() {
            eprintln!("Error flushing internal error response: {}", e);
        }
    }
}
