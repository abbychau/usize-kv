package main

import (
	"log"
	"net"
)
import "encoding/hex"

func main() {

  // connect to this socket
  conn, _ := net.Dial("tcp", "127.0.0.1:9123")
  //for {
	conn.Write([]byte{0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,1, 0,0,0,0,0,0,0,1})
	buff := make([]byte, 64)
	n, _ := conn.Read(buff)
	hex.Decode(buff, buff[:n])
	log.Printf("Receive: %s", buff )

  //}
}