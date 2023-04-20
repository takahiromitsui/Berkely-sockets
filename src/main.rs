use network_programming::{accept, bind, handle_client, listen, stream_socket, ThreadPool};

const SERVER_PORT: u16 = 3000;

pub fn start(my_addr: &nix::sys::socket::SockAddr) {
    let pool = ThreadPool::new(4);

    let sockfd = stream_socket();
    bind(sockfd, &my_addr);
    println!("[LISTENING] Listening on port: {}", my_addr);
    listen(sockfd, 10);
    println!("[STARTING] Server started");
    loop {
        let new_fd = accept(sockfd);
        println!("[ACCEPTED] Accepted connection from: {}", new_fd);
        pool.execute(move || {
            handle_client(new_fd);
        });
    }
}

fn main() {
    let server_addr = nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::new(
        nix::sys::socket::IpAddr::V4(nix::sys::socket::Ipv4Addr::new(127, 0, 0, 1)),
        SERVER_PORT,
    ));
    start(&server_addr)
}
