use super::{handle_receiver::handle_receiver, handle_sender::handle_sender};
use std::{io::Read, net::TcpStream};

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

        if request.starts_with("Receiver") {
            println!("Receiver function triggered");
            if let Err(e) = handle_receiver(&mut stream, &buffer[..input]) {
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
