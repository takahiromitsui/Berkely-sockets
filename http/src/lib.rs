use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use serde::{Deserialize, Serialize};
use serde_json;

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

pub fn post_message_json(body: &str) -> String {
    let json: Message = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(e) => {
            println!("Error: {}", e);
            return "HTTP/1.1 400 Bad Request".to_string();
        }
    };
    let response = serde_json::to_string(&json).unwrap();
    format!(
        "HTTP/1.1 200 OK\nContent-Type: application/json\nContent-Length: {}\n\n{}",
        response.len(),
        response
    )
}

pub fn get_path_from_request_line(request_line: &str) -> Option<&str> {
    let mut parts = request_line.splitn(3, " ");
    let _method = parts.next().unwrap();
    let path = parts.next().unwrap();
    Some(path)
}

pub fn get_json_from_body(body: &str) -> &str {
    //trim the null byte
    body.splitn(2, "\r\n\r\n")
        .nth(1)
        .unwrap_or("")
        .trim()
        .trim_end_matches('\0')
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let req = match parse_http_request(&buffer) {
        Some(req) => req,
        None => return,
    };
    let (request_line, _headers, body) = req;

    let response: String = if request_line.starts_with("GET") {
        let path = get_path_from_request_line(&request_line).unwrap();
        fetch_html("src/views", path)
    } else if request_line.starts_with("POST") {
        let json_string = get_json_from_body(&body);
        post_message_json(json_string)
    } else {
        println!("Unknown method");
        "HTTP/1.1 400 Bad Request".to_string()
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
