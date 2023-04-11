use client::{connect, send_message};



fn main() {
    let my_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        8080,
    ));
    let sockfd = client::stream_socket();
    connect(sockfd, &my_addr);
    send_message(sockfd, "Hello from client");
}