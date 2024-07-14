use super::allowed_request::AllowedRequest;
use crate::http::methods::list;
use crate::http::put::put;
use crate::threadpool::thread::Threadpool;
use std::error::Error;
use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct TCP;

impl TCP {
    pub async fn run(addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on {}", addr);

        let pool = Threadpool::build(6).unwrap_or_else(|_| {
            eprintln!("Failed to create thread pool");
            exit(1);
        });

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    pool.execute(|| {
                        tokio::spawn(async move {
                            if let Err(e) = TCP::handle_client(stream).await {
                                eprintln!("Error handling client: {}", e);
                            }
                        });
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = vec![0; 5_242_880];
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                println!("Connection closed by client");
                break;
            }
            let request = String::from_utf8_lossy(&buffer[..n])
                .trim_matches(char::from(0))
                .trim()
                .to_string();

            match AllowedRequest::from_str(&request) {
                Some(AllowedRequest::Put) => {
                    put(&mut stream, &mut buffer).await?;
                }
                Some(AllowedRequest::LIST) => {
                    list::list_storage(&mut stream).await?;
                }
                Some(AllowedRequest::Delete) => {
                    println!("Processing DELETE request");
                }
                Some(AllowedRequest::Get) => {
                    println!("Processing GET request");
                }
                None => {
                    println!("Unsupported request: {}", request);
                    stream.write_all(b"Unsupported request\n").await?;
                    stream.flush().await?;
                    // return Err("Unsupported request: {}".into());
                }
            }

            stream.flush().await?;
        }
        Ok(())
    }
}
