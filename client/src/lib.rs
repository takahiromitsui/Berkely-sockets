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
