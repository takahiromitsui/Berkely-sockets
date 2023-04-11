use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

const DISCONNECT_MESSAGE: &str = "!Disconnect";

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

pub fn handle_client(sockfd: i32) {
    let mut buffer = [0; 1024];
    loop {
        let res = nix::sys::socket::recv(sockfd, &mut buffer, nix::sys::socket::MsgFlags::empty());
        match res {
            Ok(_) => {
                let msg = std::str::from_utf8(&buffer).unwrap().trim_end_matches('\0');
                println!("{}", msg);
                if msg.contains(DISCONNECT_MESSAGE) {
                    nix::unistd::close(sockfd).unwrap();
                    println!("Closed connection: {}", sockfd);
                    break;
                }
            }
            Err(e) => {
                println!("Receive failed: {}", e);
                break;
            }
        }
    }
}

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

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
