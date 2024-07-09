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

#[derive(Deserialize, Debug)]
pub struct Data {
    pub data: Details,
}

#[derive(Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "screenResolution")]
    pub screen_resolution: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

pub struct TCP;

impl TCP {
    fn convert_to_json(buffer: &[u8]) -> Result<Data, std::io::Error> {
        let request = String::from_utf8_lossy(buffer);
        let re = Regex::new(r"(?s)\r\n\r\n(.*)").unwrap();
        let json_payload = re
            .captures(&request)
            .and_then(|caps| caps.get(1))
            .map_or("", |m| m.as_str());

        println!("Raw JSON payload: {}", json_payload);

        let cleaned_json = json_payload.replace(char::is_whitespace, "");

        serde_json::from_str(&cleaned_json).map_err(|e| {
            eprintln!("Error parsing JSON: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON")
        })
    }

    pub fn run(addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Server listening on {}", addr);

        let pool = Threadpool::build(6).unwrap_or_else(|e| {
            eprintln!("Failed to build thread pool: {}", e);
            exit(1);
        });

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

    fn handle_client(mut stream: TcpStream, content: Arc<Mutex<String>>) {
        let mut buffer = [0; 1024];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Connection closed by the client");
                    break;
                }
                Ok(input) => {
                    let request = String::from_utf8_lossy(&buffer[..input]);
                    let request_line = request.lines().next().unwrap_or("");
                    println!("Request: {}", request_line);

                    match request_line {
                        req if req.starts_with("GET /render/base/base.js") => {
                            TCP::handle_get_file(
                                &mut stream,
                                "render/base/base.js",
                                "application/javascript",
                            );
                        }
                        req if req.starts_with("GET /styles.css") => {
                            TCP::handle_get_file(&mut stream, "styles.css", "text/css");
                        }
                        req if req.starts_with("GET /") => {
                            TCP::handle_get(&mut stream, &content);
                        }
                        req if req.starts_with("POST /upload") => {
                            TCP::handle_post_update(&mut stream, &buffer[..input], &content);
                        }
                        req if req.starts_with("POST /receiver") => {
                            TCP::handle_post_receiver(&mut stream, &buffer[..input]);
                        }
                        req if req.starts_with("POST /sender") => {
                            match TCP::convert_to_json(&buffer) {
                                Ok(json) => {
                                    println!("{json:?}");
                                    TCP::handle_post_sender(&mut stream, &buffer[..input]);
                                }
                                Err(e) => {
                                    eprintln!("Error parsing JSON: {}", e);
                                    TCP::send_internal_error(&mut stream);
                                }
                            }
                        }
                        _ => {
                            println!("Unknown request: {}", request_line);
                            TCP::handle_unknown(&mut stream);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from stream: {}", e);
                    break;
                }
            }
        }
    }

    fn handle_get_file(stream: &mut TcpStream, file_path: &str, content_type: &str) {
        let file_content = fs::read_to_string(Path::new(file_path));
        match file_content {
            Ok(content) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    content_type,
                    content.len(),
                    content
                );
                TCP::send_response(stream, &response);
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
                TCP::send_internal_error(stream);
            }
        }
    }

    fn handle_get(stream: &mut TcpStream, content: &Arc<Mutex<String>>) {
        println!("GET function triggered");
        let response_body = content.lock().unwrap().clone();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        TCP::send_response(stream, &response);
    }

    fn handle_post_update(stream: &mut TcpStream, buffer: &[u8], content: &Arc<Mutex<String>>) {
        println!("Update function triggered");
    }

    fn handle_post_receiver(stream: &mut TcpStream, buffer: &[u8]) {
        println!("Receiver function triggered");
        if let Err(e) = handle_receiver(stream, buffer) {
            eprintln!("Error handling receiver: {}", e);
            TCP::send_internal_error(stream);
        }
    }

    fn handle_post_sender(stream: &mut TcpStream, _buffer: &[u8]) {
        println!("Sender function triggered");
        if let Err(e) = handle_sender(stream) {
            eprintln!("Error handling sender: {}", e);
            TCP::send_internal_error(stream);
        }
    }

    fn handle_unknown(stream: &mut TcpStream) {
        let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
        TCP::send_response(stream, response);
    }

    fn send_internal_error(stream: &mut TcpStream) {
        let error_response =
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\n\r\nInternal Server Error";
        TCP::send_response(stream, error_response);
    }

    fn send_response(stream: &mut TcpStream, response: &str) {
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error writing response: {}", e);
        }
        if let Err(e) = stream.flush() {
            eprintln!("Error flushing stream: {}", e);
        }
    }
}
