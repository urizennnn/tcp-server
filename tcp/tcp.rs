use crate::{http::put::put, threadpool::thread::Threadpool};
use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::allowed_request::AllowedRequest;

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
        loop {
            match stream.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buffer[..n])
                        .trim_matches(char::from(0))
                        .trim()
                        .to_string();

                    match AllowedRequest::from_str(&request) {
                        Some(AllowedRequest::Put) => {
                            put(&mut stream).await;
                            if let Err(e) = stream.write_all(b"PUT request handled").await {
                                eprintln!("Failed to write 'PUT request handled': {}", e);
                                return;
                            }
                        }
                        _ => {
                            if let Err(e) = stream.write_all(b" Invalid request").await {
                                eprintln!("Failed to write 'Invalid request': {}", e);
                                return;
                            }
                        }
                    }

                    if let Err(e) = stream.flush().await {
                        eprintln!("Failed to flush stream: {}", e);
                        return;
                    }
                }
                Ok(_) => {
                    println!("Connection closed by client");
                    break;
                }
                Err(e) => {
                    eprintln!("Failed to read from client: {}", e);
                    return;
                }
            }
        }
    }
}
