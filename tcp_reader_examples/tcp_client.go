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
	conn, _ := net.Dial("tcp", "127.0.0.1:9123")
	conn.Write([]byte{
		0, 0, 0, 0, 0, 0, 0, 4,
		0, 0, 0, 0, 0, 0, 0, 1,
		0, 0, 0, 0, 0, 0, 0, 1})
	// os.Exit(0)
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func(wg *sync.WaitGroup) {
			defer wg.Done()
			conn, _ := net.Dial("tcp", "127.0.0.1:9123")
			defer conn.Close()
			buff := make([]byte, 64)
			for i := 0; i < 10000; i++ {
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
			// conn.Read(buff)
			// hex.Decode(buff, buff)
			// log.Printf("Receive: %v", buff)
		}(&wg)
	}
	fmt.Println("Current Time", time.Now().String())

	wg.Wait()
	fmt.Println("Current Time", time.Now().String())
}
