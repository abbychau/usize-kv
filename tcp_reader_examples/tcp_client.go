package main

import (
	"fmt"
	"net"
	"sync"
	"time"
)

func main() {

	// connect to this socket
	var wg sync.WaitGroup

	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func(wg *sync.WaitGroup) {
			defer wg.Done()
			conn, _ := net.Dial("tcp", "127.0.0.1:9123")
			buff := make([]byte, 64)
			for i := 0; i < 1000; i++ {
				conn.Write([]byte{
					0, 0, 0, 0, 0, 0, 0, 2,
					0, 0, 0, 0, 0, 0, 0, 1,
					0, 0, 0, 0, 0, 0, 0, 1})

				conn.Read(buff)
				conn.Write([]byte{
					0, 0, 0, 0, 0, 0, 0, 1,
					0, 0, 0, 0, 0, 0, 0, 1,
					0, 0, 0, 0, 0, 0, 0, 2})
				conn.Read(buff)
				conn.Write([]byte{
					0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 1,
					0, 0, 0, 0, 0, 0, 0, 0})
				conn.Read(buff)
			}
			// buff := make([]byte, 64)
			// conn.Read(buff)
			// hex.Decode(buff, buff[:n])
			// log.Printf("Receive: %v", buff)
		}(&wg)
	}
	fmt.Println("Current Time", time.Now().String())

	wg.Wait()
	fmt.Println("Current Time", time.Now().String())
}
