use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read or get client");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request{}", request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(Path::new("hello.html")).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
