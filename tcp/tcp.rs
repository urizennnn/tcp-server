use super::{handle_receiver::handle_receiver, handle_sender::handle_sender};
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        // Read from the TCP stream
        let bytes_read = match stream.read(&mut buffer) {
            Ok(size) => size,
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        };

        if bytes_read == 0 {
            println!("Connection closed by the client");
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Extract the body
        let body = extract_body(&request);

        if request.starts_with("GET") {
            handle_get_request(&mut stream);
        } else if request.starts_with("POST") {
            println!("POST request received: {}", body);
            handle_post_request(&mut stream, &body);
        } else if request.starts_with("Receiver") {
            println!("Receiver function triggered");
            if let Err(e) = handle_receiver(&mut stream, &buffer[..bytes_read]) {
                eprintln!("Error handling receiver: {}", e);
            }
        } else if request.starts_with("Sender") {
            println!("Sender function triggered");
            if let Err(e) = handle_sender(&mut stream, &mut buffer) {
                eprintln!("Error handling sender: {}", e);
            }
        } else {
            println!("Unknown request: {}", request);
        }
    }
}

fn extract_body(request: &str) -> &str {
    // Find the index of the empty line that separates headers from the body
    if let Some(body_index) = request.find("\r\n\r\n") {
        &request[(body_index + 4)..]
    } else {
        ""
    }
}

fn handle_get_request(stream: &mut TcpStream) {
    let content = match fs::read_to_string(Path::new("render/base.html")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error sending response: {}", e);
    }

    if let Err(e) = stream.flush() {
        eprintln!("Error flushing stream: {}", e);
    }

    println!("GET request handled");
}

fn handle_post_request(stream: &mut TcpStream, body: &str) {
    // Process the POST request body (e.g., log it, send a response, etc.)
    println!("POST body: {}", body);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error sending response: {}", e);
    }
}
