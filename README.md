# Network Programming with Rust
This Rust project is a simple implementation of a server using Berkeley sockets. It contains a server and a client, and the server listens for incoming connections and handles them using worker threads in a thread pool. The implementation uses **TCP** and **UDP** sockets to handle incoming data from connected clients.

# Diagram (sockets)
![socket_diagram](https://user-images.githubusercontent.com/78789212/233333063-0a002883-0f1a-4306-841a-e07657489505.png)


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
### TCP
**The left side is a server and the right side is a client.**
<img width="852" alt="Screenshot 2023-05-03 at 13 30 35" src="https://user-images.githubusercontent.com/78789212/235904607-2e86c5dc-aac1-4b13-98ff-f7941ef98149.png">


### UDP
**The left side is a server and the right side is a client.**
<img width="860" alt="Screenshot 2023-05-03 at 13 31 29" src="https://user-images.githubusercontent.com/78789212/235904636-bf73bea6-94de-4063-87b9-7782aee1bdce.png">


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

`handle_tcp`

This function handles incoming data from a connected client by TCP. It receives data in 1024 byte chunks, prints the data to the console, and closes the connection if the data contains the disconnect message.

`handle_udp`

This function handles incoming data from a connected client by UDP. It receives data in 1024 byte chunks, prints the data to the console, and closes the connection if the data contains the disconnect message.

`start`

This function starts the server. It creates a new ThreadPool object to handle incoming connections, creates a new socket, binds the socket to the server's address, and listens on the socket for incoming connections. For each incoming connection, it accepts the connection and passes the connection's file descriptor to a worker thread in the thread pool. The worker thread calls the handle_tcp or handle_udp function to handle the connection.

`ThreadPool`

This struct represents a thread pool for handling incoming connections. It contains a vector of Worker objects and a sender channel for passing jobs to the worker threads.

`Worker`

This struct represents a worker thread in the thread pool. It contains a thread ID and a joinable thread handle. The new function creates a new worker thread that listens for jobs on the thread pool's receiver channel. When a job is received, the worker thread executes the job.

### client/main.rs
The main.rs file contains the entry point for the client and sends messages to the server using the connect and send_message functions from lib.rs.

### client/lib.rs
The lib.rs file contains the implementation of the stream_socket, connect, and send_message functions used by the client. The stream_socket function creates a TCP socket, while the connect function connects the socket to the server. The send_message function sends a message to the server.
