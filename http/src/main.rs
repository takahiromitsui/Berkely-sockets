use std::net::TcpListener;

use http::{handle_connection, ThreadPool};

fn main() {
    let pool = ThreadPool::new(4);
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}
