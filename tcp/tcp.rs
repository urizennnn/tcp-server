use super::allowed_request::AllowedRequest;
use crate::http::methods::list;
use crate::http::put::put;
use crate::threadpool::thread::Threadpool;
use std::error::Error;
use std::process::exit;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct TCP;

impl TCP {
    pub async fn run(addr: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on {}", addr);

        let pool = Arc::new(Threadpool::build(6).unwrap_or_else(|_| {
            eprintln!("Failed to create thread pool");
            exit(1);
        }));

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let pool = Arc::clone(&pool);
                    pool.execute(move || {
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
            let n = match stream.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    continue;
                }
            };

            if n == 0 {
                println!("Connection closed by client");
                break;
            }

            let request = Self::parse_request(&buffer, n)?;

            match AllowedRequest::from_str(&request) {
                Some(AllowedRequest::Put) => Self::process_put(&mut stream, &mut buffer).await?,
                Some(AllowedRequest::List) => Self::process_list(&mut stream).await?,
                Some(AllowedRequest::Delete) => Self::process_delete(&request),
                Some(AllowedRequest::Get) => Self::process_get(&request),
                None => {
                    eprintln!("Unsupported request: {}", request);
                    stream.write_all(b"Unsupported request\n").await?;
                }
            }

            if let Err(e) = stream.flush().await {
                eprintln!("Failed to flush stream: {}", e);
            }
        }
        Ok(())
    }

    fn parse_request(buffer: &[u8], n: usize) -> Result<String, Box<dyn Error>> {
        let request = String::from_utf8_lossy(&buffer[..n])
            .trim_matches(char::from(0))
            .trim()
            .to_string();
        Ok(request)
    }

    async fn process_put(
        stream: &mut TcpStream,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        put(stream, buffer).await?;
        Ok(())
    }

    async fn process_list(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        list::list_storage(stream).await?;
        Ok(())
    }

    fn process_delete(request: &str) {
        println!("Processing DELETE request: {}", request);
        // Implement actual delete logic here
    }

    fn process_get(request: &str) {
        println!("Processing GET request: {}", request);
        // Implement actual get logic here
    }
}
