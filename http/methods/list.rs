use std::error::Error;
use std::fs;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn list_storage(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    stream.write_all(b"Listing storage items...\n").await?;

    let mut content = String::new();
    let entries = fs::read_dir("storage")?;
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        content.push_str(&file_name_str);
        content.push('\n');
    }

    stream.write_all(content.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
