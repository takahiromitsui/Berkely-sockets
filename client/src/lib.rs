use std::{fs::File, io::{BufReader, Read}, path::Path};

pub fn stream_socket() -> i32 {
    // AF_INET = IPv4
    let domain = nix::sys::socket::AddressFamily::Inet;
    // SOCK_STREAM = TCP
    let socket_type = nix::sys::socket::SockType::Stream;
    // Protocol = TCP
    let protocol = nix::sys::socket::SockProtocol::Tcp;

    // Additional flags
    let flags = nix::sys::socket::SockFlag::empty();

    let fd = nix::sys::socket::socket(domain, socket_type, flags, protocol).unwrap();
    println!("Created socket with fd: {}", fd);
    fd
}

pub fn connect(sockfd: i32, my_addr: &nix::sys::socket::SockAddr) -> i32 {
    let res = nix::sys::socket::connect(sockfd, my_addr);
    match res {
        Ok(_) => {
            println!("Connect successful");
            0
        }
        Err(e) => {
            println!("Connect failed: {}", e);
            -1
        }
    }
}

pub fn send_message(sockfd: i32, message: &str) -> i32 {
    let res = nix::sys::socket::send(
        sockfd,
        message.as_bytes(),
        nix::sys::socket::MsgFlags::empty(),
    );
    match res {
        Ok(_) => {
            println!("Send successful");
            0
        }
        Err(e) => {
            println!("Send failed: {}", e);
            -1
        }
    }
}

pub fn send_file(sockfd: i32, file_path: &str) -> i32 {
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return -1;
        }
    };

    let mut buffer = [0; 1024];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                match nix::sys::socket::send(
                    sockfd,
                    &buffer[0..n],
                    nix::sys::socket::MsgFlags::empty(),
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error sending file: {}", e);
                        return -1;
                    }
                };
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                return -1;
            }
        }
    }

    println!("File sent successfully");
    0
}
    

