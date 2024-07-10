use crate::threadpool::thread::Threadpool;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};

pub struct TCP;

impl TCP {
    pub fn run(addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind to address");
        println!("Server listening on {}", addr);

        let pool = Threadpool::build(2).unwrap_or_else(|e| {
            eprintln!("Failed to build thread pool: {}", e);
            exit(1);
        });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    pool.execute(move || {
                        TCP::handle_client(stream);
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        eprintln!("Server shutdown.");
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("Failed to read or get client");
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Received request{}", request);
        stream.write_all(b"Hello from TCSHARE").unwrap();
        stream.flush().unwrap();
        return;
    }
}
