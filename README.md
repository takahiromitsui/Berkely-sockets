# Network Programming with Rust
This project implements a simple server using Rust and Berkeley sockets.

## Run Locally
- Clone project
- Run server
```bash
  cargo run
```
- Run client
```bash
  cd client
  cargo run
```
The server listens on 127.0.0.1:8080 by default.

## Example
**The left side is a server and the right side is a client.**

![Screenshot 2023-04-11 at 15 20 48](https://user-images.githubusercontent.com/78789212/231175915-328c8290-dde1-4397-b0ed-d2757223b3eb.png)



## Implementation
### src/main.rs
The main.rs file contains the entry point of the program. It creates a new SockAddr object representing the server's address, and starts the server by calling the start function from the lib.rs module.

### src/lib.rs
The lib.rs module contains the implementation of the server.

`stream_socket`

This function creates a new TCP socket and returns its file descriptor.

`bind`

This function binds the socket to a given address.

`listen`

This function listens on the socket for incoming connections.

`accept`

This function accepts an incoming connection and returns a new file descriptor representing the connection.

`handle_client`

This function handles incoming data from a connected client. It receives data in 1024 byte chunks, prints the data to the console, and closes the connection if the data contains the disconnect message.

`start`

This function starts the server. It creates a new ThreadPool object to handle incoming connections, creates a new socket, binds the socket to the server's address, and listens on the socket for incoming connections. For each incoming connection, it accepts the connection and passes the connection's file descriptor to a worker thread in the thread pool. The worker thread calls the handle_client function to handle the connection.

`ThreadPool`

This struct represents a thread pool for handling incoming connections. It contains a vector of Worker objects and a sender channel for passing jobs to the worker threads.

`Worker`

This struct represents a worker thread in the thread pool. It contains a thread ID and a joinable thread handle. The new function creates a new worker thread that listens for jobs on the thread pool's receiver channel. When a job is received, the worker thread executes the job.

### client/main.rs
The main.rs file contains the entry point for the client and sends messages to the server using the connect and send_message functions from lib.rs.

### client/lib.rs
The lib.rs file contains the implementation of the stream_socket, connect, and send_message functions used by the client. The stream_socket function creates a TCP socket, while the connect function connects the socket to the server. The send_message function sends a message to the server.
