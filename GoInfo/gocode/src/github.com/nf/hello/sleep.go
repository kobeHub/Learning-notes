package main

import (
	"fmt"
	"time"
)

func dothings(weights int, stop chan int) {
	for {
		fmt.Println(weights, "do things")
		select {
		case pri := <-stop:
				if pri > weights {
					fmt.Printf("weights %v exit\n", weights)
					return

			}
		}
	}
}

func demo(x int) {
	stop := make(chan int)
	for i := 0; i < 10; i ++ {
		go dothings(i, stop)
	}
	time.Sleep(2 * time.Second)
	stop<-x
}

func main() {
	demo(5)
}
