use std::net::UdpSocket;

use client::{bind, connect, send_file, send_tcp_message, send_udp_message};
use nix::sys::socket::SockAddr;

const SERVER_PORT: u16 = 3000;
const CLIENT_PORT: u16 = 8080;
const DISCONNECT_MESSAGE: &str = "!Disconnect";

pub enum ProtocolType {
    TCP,
    UDP,
}

pub fn handle_tcp(client_addr: SockAddr, server_addr: SockAddr) {
    println!("[TCP]");
    println!("[CREATING] Creating client socket");
    let sockfd = client::stream_socket();
    println!("[BINDING] Binding client: {}", client_addr);
    bind(sockfd, &client_addr);
    println!("[CONNECTING] Connecting to server: {}", server_addr);
    connect(sockfd, &server_addr);
    send_tcp_message(sockfd, "Message 1 from client\n");
    send_tcp_message(sockfd, "Message 2 from client\n");
    send_tcp_message(sockfd, "Message 3 from client\n");
    send_tcp_message(sockfd, "Message 4 from client\n");
    send_file(sockfd, "src/test.txt");
    send_tcp_message(sockfd, DISCONNECT_MESSAGE);
}

pub fn handle_udp(client_addr: SockAddr, server_addr: SockAddr) {
    println!("[UDP]");
    println!("[CREATING] Creating client socket");
    println!("[BINDING] Binding client: {}", client_addr);
    let sockfd = match UdpSocket::bind(client_addr.to_str()) {
        Ok(socket) => socket,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };
    send_udp_message(&sockfd, &server_addr.to_str(), "Message 1 from client\n");
    send_udp_message(&sockfd, &server_addr.to_str(), "Message 2 from client\n");
    send_udp_message(&sockfd, &server_addr.to_str(), "Message 3 from client\n");
    send_udp_message(&sockfd, &server_addr.to_str(), "Message 4 from client\n");
    // In UDP, since there is no connection, the socket is only used to send and receive datagrams. Once you are done with sending and receiving, you don't need to explicitly close the socket.
    // send_udp_message(&sockfd, &server_addr.to_str(), DISCONNECT_MESSAGE);
}

pub fn start(client_addr: SockAddr, server_addr: SockAddr, protocol: ProtocolType) {
    match protocol {
        ProtocolType::TCP => handle_tcp(client_addr, server_addr),
        ProtocolType::UDP => handle_udp(client_addr, server_addr),
    };
}

fn main() {
    let client_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        CLIENT_PORT,
    ));
    let server_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        SERVER_PORT,
    ));
    // start(client_addr, server_addr, ProtocolType::TCP);
    start(client_addr, server_addr, ProtocolType::UDP);
}
