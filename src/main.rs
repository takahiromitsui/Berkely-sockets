use network_programming::{
    accept, bind, datagram_socket, handle_client, listen, stream_socket, ThreadPool,
};
use nix::sys::socket::SockAddr;

const SERVER_PORT: u16 = 3000;

pub enum ProtocolType {
    TCP,
    UDP,
}

pub fn handle_tcp(server_addr: &SockAddr) {
    let pool = ThreadPool::new(4);
    let sockfd = stream_socket();
    bind(sockfd, server_addr);
    println!("[TCP]");
    println!("[LISTENING] Listening: {}", server_addr);
    listen(sockfd, 10);
    println!("[STARTING] Server started");
    loop {
        let new_fd = accept(sockfd);
        if new_fd == -1 {
            nix::unistd::close(sockfd).unwrap();
            println!("[FAILED] Closed connection: {}", sockfd);
            break;
        }
        println!("[ACCEPTED] Accepted connection from: {}", new_fd);
        pool.execute(move || {
            handle_client(new_fd);
        });
    }
}

pub fn handle_udp(server_addr: &SockAddr) {
    let pool = ThreadPool::new(4);
    let sockfd = datagram_socket();
    bind(sockfd, server_addr);
    println!("[UDP]");
    println!("[LISTENING] Listening: {}", server_addr);
    println!("[STARTING] Server started");
    loop {
        pool.execute(move || {
            handle_client(sockfd);
        });
    }
}

pub fn start(server_addr: &SockAddr, protocol: ProtocolType) {
    match protocol {
        ProtocolType::TCP => handle_tcp(server_addr),
        ProtocolType::UDP => handle_udp(server_addr),
    };
}

fn main() {
    let server_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        SERVER_PORT,
    ));
    // start(&server_addr, ProtocolType::TCP);
    start(&server_addr, ProtocolType::UDP);
}
