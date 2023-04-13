use std::{net::TcpStream, io::{Write, Read}};

pub fn handle_client(mut stream: TcpStream) {
    let _ = stream.write_all(b"220 Welcome to my FTP server\r\n");
    loop {
        let mut command =String::new();
        let _ = stream.read_to_string(&mut command);
        let command = command.trim();
        match command.to_uppercase().as_str() {
            "USER" => {
                let _ = stream.write_all(b"331 Please specify the username.\r\n");
                let mut username = String::new();
                let _ = stream.read_to_string(&mut username);
                println!("Username: {}", username.trim());
                let _ = stream.write_all(b"331 Please specify the password.\r\n");
            }
            "PASS" => {
                let _ = stream.write_all(b"230 Login successful.\r\n");
            }
            "RETR" => {
                let _ = stream.write_all(b"150 Opening data connection.\r\n");
                let file_data = "Hello, world!".as_bytes().to_vec();
                let _ = stream.write_all(&file_data);
                let _ = stream.write_all(b"\r\n226 Transfer complete.\r\n");
            }
            "QUIT" => {
                let _ = stream.write_all(b"221 Goodbye.\r\n");
                break;
            }
            _ => {
                let _ = stream.write_all(b"502 Command not implemented.\r\n");
            }
        }
    }
}