use std::net::TcpListener;
use std::process;

use tcp::tcp::tcp::handle_client;
use tcp::threadpool::thread::Threadpool;

fn main() {
    // Bind TCP listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080");

    // Build the thread pool
    let pool = match Threadpool::build(4) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to build thread pool: {}", e);
            process::exit(1);
        }
    };

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    println!("Server shutdown.");
}
