use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    guest: String,
}

pub fn parse_http_request(buffer: &[u8]) -> Option<(String, Vec<(String, String)>, String)> {
    let mut headers_start = None;
    let mut headers_end = None;
    let mut body_start = None;
    for (i, chunk) in buffer.windows(2).enumerate() {
        if chunk == b"\r\n" {
            if headers_start.is_none() {
                headers_start = Some(i + 2);
            } else if headers_end.is_none() {
                headers_end = Some(i);
                body_start = Some(i + 2);
                break;
            }
        }
    }
    if let (Some(start), Some(end), Some(body_start)) = (headers_start, headers_end, body_start) {
        let request_line = String::from_utf8_lossy(&buffer[..start - 2]).into_owned();
        let headers = String::from_utf8_lossy(&buffer[start..end]).into_owned();
        let body = String::from_utf8_lossy(&buffer[body_start..]).into_owned();
        let headers = headers
            .split("\r\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.splitn(2, ": ");
                (
                    parts.next().unwrap().to_owned(),
                    parts.next().unwrap().to_owned(),
                )
            })
            .collect();
        Some((request_line, headers, body))
    } else {
        None
    }
}

pub fn fetch_html(root: &str, path: &str) -> String {
    let mut file_path = root.to_string();
    if path == "/" {
        file_path.push_str("/index.html");
    } else {
        file_path.push_str(&format!("{}.html", path));
    }

    let file = std::fs::read_to_string(file_path);
    let not_found = std::fs::read_to_string(format!("{}/404.html", root));

    match file {
        Ok(body) => {
            format!(
                "HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: {}\n\n{}",
                body.len(),
                body
            )
        }
        Err(_) => {
            let body = match not_found {
                Ok(body) => body,
                Err(_) => "404 Not Found".to_string(),
            };
            format!(
                "HTTP/1.1 404 Not Found\nContent-Type: text/html\nContent-Length: {}\n\n{}",
                body.len(),
                body
            )
        }
    }
}

pub fn post_message_json(body: &str){
    let json: Message = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    // println!("json: {:?}", json);

    // let message = format!("{}: {}", json.guest, json.message);
    // let mut messages = std::fs::read_to_string("src/messages.txt").unwrap_or_else(|_| "".to_string());
    // messages.push_str(&message);
    // messages.push_str("\n");

    // std::fs::write("src/messages.txt", messages).unwrap();
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let req = match parse_http_request(&buffer) {
        Some(req) => req,
        None => return,
    };
    let (request_line, headers, body) = req;
    // println!("Request line: {}", request_line);
    // println!("Headers: {:?}", headers);
    println!("Body: {}", body);

    let response: String = if request_line.starts_with("GET") {
        let mut parts = request_line.splitn(3, " ");
        let _method = parts.next().unwrap();
        let path = parts.next().unwrap();
        fetch_html("src/views", path)
    } else if request_line.starts_with("POST") {
        // trim the null byte
        let json_start = body.splitn(2, "\r\n\r\n").nth(1).unwrap_or("").trim().trim_end_matches('\0');
        post_message_json(json_start);

        "HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: 0\n\n".to_string()
    } else {
        println!("Unknown method");
        "HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: 0\n\n".to_string()
    };
    stream.write(response.as_bytes()).unwrap();
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
