use std::{fs, io, path::Path};

pub fn handle_sender(buffer: &[u8]) -> Result<String, io::Error> {
    println!(
        "Sender function called with: {}",
        String::from_utf8_lossy(buffer)
    );

    // Read the sender.html file
    let content = match fs::read_to_string(Path::new("render/sender.html")) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading sender.html: {}", e);
            return Err(e);
        }
    };

    // Construct the HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    Ok(response)
}
