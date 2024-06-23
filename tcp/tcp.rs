use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read or get client");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request{}", request);

    let response = "Hello client".as_bytes();
    stream.write(response).expect("Failed to write response");
}
