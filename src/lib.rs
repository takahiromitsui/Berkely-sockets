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

pub fn bind(sockfd: i32, my_addr: &nix::sys::socket::SockAddr) -> i32 {
    let res = nix::sys::socket::bind(sockfd, my_addr);
    match res {
        Ok(_) => {
            println!("Bind successful");
            0
        }
        Err(e) => {
            println!("Bind failed: {}", e);
            -1
        }
    }
}

pub fn listen(sockfd: i32, backlog: usize) -> i32 {
    let res = nix::sys::socket::listen(sockfd, backlog);
    match res {
        Ok(_) => {
            println!("Listen successful");
            0
        }
        Err(e) => {
            println!("Listen failed: {}", e);
            -1
        }
    }
}

pub fn accept(sockfd: i32) -> i32 {
    let res = nix::sys::socket::accept(sockfd);
    match res {
        Ok(fd) => {
            println!("Accept successful");
            fd
        }
        Err(e) => {
            println!("Accept failed: {}", e);
            -1
        }
    }
    
}

pub fn start(my_addr: &nix::sys::socket::SockAddr) {
    let sockfd = stream_socket();
    bind(sockfd, &my_addr);
    println!("[LISTENING] Listening on port: {}", my_addr);
    listen(sockfd, 10);
    println!("[STARTING] Server started");
    loop {
        let new_fd = accept(sockfd);
        println!("[ACCEPTED] Accepted connection from: {}", new_fd);
    }
}
