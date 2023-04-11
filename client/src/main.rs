use client::{connect, send_message};

const DISCONNECT_MESSAGE: &str = "!Disconnect";

fn main() {
    let my_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        8080,
    ));
    let sockfd = client::stream_socket();
    connect(sockfd, &my_addr);
    send_message(sockfd, "Message 1 from client\n");
    send_message(sockfd, "Message 2 from client\n");
    send_message(sockfd, "Message 3 from client\n");
    send_message(sockfd, "Message 4 from client\n");
    send_message(sockfd, DISCONNECT_MESSAGE);
}
