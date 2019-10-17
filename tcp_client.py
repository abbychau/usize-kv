import socket
import sys
import threading
import time

def call(threadName, times):

    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(('127.0.0.1', 9123))

    param = bytearray.fromhex(sys.argv[1]) + bytearray.fromhex(sys.argv[2]) + bytearray.fromhex(sys.argv[3])

    for _ in range(times):
        #print(param.hex())

        client.send(param)
        response = client.recv(64)

        #print(response.hex())
    
    # print("%s Ended: %s" % ( threadName, time.ctime(time.time()) ))


num = 1000
th = {}
for i in range(100):
    th[i] = threading.Thread( target=call, args=("Thread-"+str(i),num) )
    th[i].start()

print("Started: %s" % (time.ctime(time.time()) ))
for i in range(100):
    th[i].join()
print("Ended: %s" % ( time.ctime(time.time()) ))
