use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn put(stream: &mut TcpStream) {
    println!("You have reached the PUT function");
    if let Err(e) = stream.write_all(b"Reached ").await {
        eprintln!("Failed to write 'Reached': {}", e);
        return;
    }
    if let Err(e) = stream.flush().await {
        eprintln!("Failed to flush stream: {}", e);
    }
}
