use std::net::TcpListener;

use tcp::{tcp::tcp::handle_client, threadpool::thread::Threadpool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080");
    let pool = Threadpool::build(6).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(e) => {
                eprintln!("Failed to establish connection {}", e)
            }
        }
    }
}
