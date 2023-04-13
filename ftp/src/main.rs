use std::{io::Write, net::TcpListener};

fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let _ = stream.write_all(b"220 Welcome to my FTP server\r\n");
    }
}
