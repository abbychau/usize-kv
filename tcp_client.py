import socket
import sys
import threading
from datetime import datetime

def justHex64(istr):
    return bytearray.fromhex(istr.rjust(16,'0'))


client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('127.0.0.1', 9123))
def call(a,b,c, times):
    param = justHex64(a) + justHex64(b) + justHex64(c)

    for _ in range(times):
        #print(param.hex())

        client.send(param)
        response = client.recv(64)
        # print("RESULT:")
        # print(response.hex())
        # print("-----\n")
        
    
    # print("%s Ended: %s" % ( threadName, time.ctime(time.time()) ))
# action key value
# call("0000000000000002", "0000000000000001", "0000000000000002",1) #UPDATE 2 to 1
# call("0000000000000000", "0000000000000001", "0000000000000001",1) #READ 1
# call("0000000000000001", "0000000000000001", "0000000000000123",1) #APPEND 123 to 1
# call("0000000000000000", "0000000000000001", "0000000000000000",1) #READ 1
# call("0000000000000002", "0000000000000001", "0000000000000002",1) #UPDATE 2 to 1
# call("0000000000000000", "0000000000000001", "0000000000000000",1) #READ 1
# call("0000000000000003", "0000000000000001", "0000000000000000",1) #REMOVE 1
# call("0000000000000000", "0000000000000001", "0000000000000000",1) #READ 1
num = 1000
threadnum = 2
th = {}
for i in range(threadnum):
    th[i] = threading.Thread( target=call, args=("0000000000000000", "0000000000000001", "0000000000000000",num) )
    th[i].start()

print("Started: %s" % datetime.utcnow().isoformat(sep=' ', timespec='milliseconds'))
for i in range(threadnum):
    th[i].join()
print("Ended: %s" % datetime.utcnow().isoformat(sep=' ', timespec='milliseconds'))
