import socket
import sys
import threading
from datetime import datetime

def justHex64(istr):
    return bytearray.fromhex(istr.rjust(16,'0'))

def call(threadName, times):

    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(('127.0.0.1', 9123))

    param = justHex64(sys.argv[1]) + justHex64(sys.argv[2]) + justHex64(sys.argv[3])

    for _ in range(times):
        #print(param.hex())

        client.send(param)
        client.recv(1)

        #print(response.hex())
    
    # print("%s Ended: %s" % ( threadName, time.ctime(time.time()) ))


num = 100
threadnum = 100
th = {}
for i in range(threadnum):
    th[i] = threading.Thread( target=call, args=("Thread-"+str(i),num) )
    th[i].start()

print("Started: %s" % datetime.utcnow().isoformat(sep=' ', timespec='milliseconds'))
for i in range(threadnum):
    th[i].join()
print("Ended: %s" % datetime.utcnow().isoformat(sep=' ', timespec='milliseconds'))
