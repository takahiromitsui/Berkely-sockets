use client::{bind, connect, send_file, send_message};

const SERVER_PORT: u16 = 3000;
const CLIENT_PORT: u16 = 8080;
const DISCONNECT_MESSAGE: &str = "!Disconnect";

fn main() {
    let client_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        CLIENT_PORT,
    ));
    let server_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        SERVER_PORT,
    ));
    let sockfd = client::stream_socket();
    bind(sockfd, &client_addr);
    connect(sockfd, &server_addr);
    send_message(sockfd, "Message 1 from client\n");
    send_message(sockfd, "Message 2 from client\n");
    send_message(sockfd, "Message 3 from client\n");
    send_message(sockfd, "Message 4 from client\n");
    send_file(sockfd, "src/test.txt");
    send_message(sockfd, DISCONNECT_MESSAGE);
}
