use crate::{http::put::put, threadpool::thread::Threadpool};
use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct TCP;

impl TCP {
    pub async fn run(addr: &str) {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");
        println!("Server listening on {}", addr);
        let pool = Threadpool::build(6).unwrap_or_else(|_| {
            eprintln!("Failed to create thread pool");
            exit(1);
        });

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    pool.execute(move || {
                        tokio::spawn(async move {
                            TCP::handle_client(stream).await;
                        });
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    async fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if let Err(e) = stream.read(&mut buffer).await {
            eprintln!("Failed to read from client: {}", e);
            return;
        }

        // Trim null characters and whitespace
        let request = String::from_utf8_lossy(&buffer[..])
            .trim_matches(char::from(0))
            .trim()
            .to_string();

        match request.as_str() {
            "PUT" => {
                put(&mut stream).await;
                if let Err(e) = stream.write_all(b"Testing").await {
                    eprintln!("Failed to write 'Testing': {}", e);
                    return;
                }
                if let Err(e) = stream.flush().await {
                    eprintln!("Failed to flush stream: {}", e);
                    return;
                }
            }
            _ => {
                let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write response: {}", e);
                }
            }
        }
    }
}
