use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn list_storage(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    stream.write_all(b"Listing storage items...\n").await?;
    stream.flush().await?;
    Ok("Storage items listed".to_string())
}
