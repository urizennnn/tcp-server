use std::error::Error;

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn put(stream: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    println!("Processing PUT request");

    loop {
        let buf = stream.read(buffer).await?;
        let buf_string = String::from_utf8_lossy(&buffer[..buf])
            .trim_matches(char::from(0))
            .trim()
            .to_string();
        let parts: Vec<&str> = buf_string.split_whitespace().collect();

        println!("{parts:?}");
        if parts.len() != 3 || parts[0] != "UPLOAD" {
            return Err("Invalid upload command".into());
        }

        let file_name = parts[1];
        let file_size: u64 = parts[2].parse()?;
        println!("Uploading file: {} ({}  bytes)", file_name, file_size);

        let mut file = File::create(file_name).await?;
        let mut remaining = file_size;

        while remaining > 0 {
            let to_read = std::cmp::min(buffer.len() as u64, remaining) as usize;
            let bytes_read = stream.read(&mut buffer[..to_read]).await?;

            if bytes_read == 0 {
                return Err("Unexpected end of file".into());
            }
            file.write_all(&buffer[..bytes_read]).await?;
            remaining -= bytes_read as u64;
        }
        println!("File upload completed");
        stream.write_all(b"File uploaded successfully\n").await?;
        stream.flush().await?;
        break;
    }
    Ok(())
}
