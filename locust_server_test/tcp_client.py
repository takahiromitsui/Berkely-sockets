from locust import User, task
from gevent import socket


class TcpClient:
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.connect((self.host, self.port))

    def send(self, data):
        self.socket.sendall(data)
        response = self.socket.recv(1024)
        return response

    def close(self):
        self.socket.close()


class TcpTaskSet(User):
    def on_start(self):
        self.client = TcpClient("127.0.0.1", 8080)

    def on_stop(self):
        self.client.close()

    @task
    def send_message(self):
        message = "Hello, world!"
        response = self.client.send(message.encode("utf-8"))
        if response:
            print(f'Response received: {response.decode("utf-8")}')
        else:
            print("No response received.")
