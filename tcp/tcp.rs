use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let upload = b"POST /upload";
    let download = b"DOWNLOAD ";

    if buffer.starts_with(get) {
        // Serve the static webpage
        println!("{:?}", String::from_utf8_lossy(&buffer));
        let contents = fs::read_to_string(Path::new("hello.html")).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(upload) {
        println!("{:?}", String::from_utf8_lossy(&buffer));
        // Handle file upload
        let filename = &buffer[7..].split(|&x| x == b' ').next().unwrap();
        let filename = std::str::from_utf8(filename).unwrap();
        let mut file = fs::File::create(filename).unwrap();
        stream.read(&mut buffer).unwrap();
        file.write_all(&buffer).unwrap();
    } else if buffer.starts_with(download) {
        // Handle file download
        let filename = &buffer[9..].split(|&x| x == b' ').next().unwrap();
        let filename = std::str::from_utf8(filename).unwrap();
        let contents = fs::read(filename).unwrap();
        stream.write_all(&contents).unwrap();
    }
}
