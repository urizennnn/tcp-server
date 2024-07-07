use std::{
    fs,
    io::{self, Write},
    net::TcpStream,
    path::Path,
};

pub fn handle_receiver(stream: &mut TcpStream, _buffer: &[u8]) -> Result<(), io::Error> {
    let content = match fs::read_to_string(Path::new("render/receiver/receiver.html")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading receiver.html: {}", e);
            return Err(e);
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
