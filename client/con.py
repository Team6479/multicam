#!/usr/bin/env python

import socket

TCP_IP = "127.0.0.1"
TCP_PORT = 5800
BUFFER_SIZE = 2048

f = open("test.h264", "a")

while True:
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((TCP_IP, TCP_PORT))
    s.send(bytes(0))
    f.write(str(s.recv(BUFFER_SIZE)))

print(data)
