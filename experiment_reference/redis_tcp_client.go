package main

import (
	"fmt"
	"log"
	"sync"
	"time"

	// Import the redigo/redis package.
	"github.com/gomodule/redigo/redis"
)

func makeTimestamp() int64 {
	return time.Now().UnixNano() / int64(time.Millisecond)
}
func main() {
	var wg sync.WaitGroup

	gn := 100
	wg.Add(gn)

	//_, err = conn.Do("HMSET", "album:2", "title", "Electric Ladyland", "artist", "Jimi Hendrix", "price", 4.95, "likes", 8)
	fmt.Printf("%d \n", makeTimestamp())
	for i := 0; i < gn; i++ {
		go func() {
			defer wg.Done()
			conn, err := redis.Dial("tcp", "localhost:6379")
			defer conn.Close()
			if err != nil {
				log.Fatal(err)
			}
			for j := 0; j < 1000; j++ {
				conn.Send("GET", "1")
				conn.Flush()
				_, err = conn.Receive()

				if err != nil {
					log.Fatal(err)
				}
			}
			//fmt.Println(r) // prints [1, 1]
		}()

	}
	wg.Wait()

	fmt.Printf("%d \n", makeTimestamp())
}
