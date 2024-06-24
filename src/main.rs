use std::net::TcpListener;

use tcp::{tcp::tcp::handle_client, threadpool::thread::Threadpool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on port 8080");
    let pool = Threadpool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream);
        })
    }
    println!("Shutting Downn");
}
