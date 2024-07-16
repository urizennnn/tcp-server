use std::{error::Error, path::Path};

use log::info;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn get_file(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
    info!("Getting file..");
    let buf_string = String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .trim()
        .to_string();
    let parts: Vec<&str> = buf_string.split_whitespace().collect();

    if parts.len() != 2 {
        stream.write_all(b"Invalid Command").await?;
        return Err("Invalid command".into());
    }
    let raw_path = parts[1];
    let format_path = format!("storage/{raw_path}");
    let path = Path::new(&format_path);
    let file = File::open(&path).await?;
    let file_size = file.metadata().await?.len();
    let response = format!("Details:{format_path:?} {:?}", file_size);
    println!("{response}");
    stream.write_all(response.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;
    info!("File sent");
    Ok(())
}
