import socket
import time
import random

sender_socket = socket.socket()

sender_socket.bind(("127.0.0.1", 8080))

sender_socket.listen()

(s, h_ip) = sender_socket.accept()

i = 0
while True:
    s.send(str(i).encode())    
    print(i)
    time.sleep(1)
    i += 1
    if(random.randint(0,9) == 3):
        time.sleep(100)
