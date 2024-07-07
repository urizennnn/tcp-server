use std::{
    fs,
    io::{self, Write},
    net::TcpStream,
    path::Path,
};

pub fn handle_sender(stream: &mut TcpStream) -> Result<(), io::Error> {
    let content = match fs::read_to_string(Path::new("render/sender/sender.html")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading sender.html: {}", e);
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
