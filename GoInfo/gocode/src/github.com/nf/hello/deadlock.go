package main

import (
	"fmt"
	"sync"
)

var wg sync.WaitGroup

func consumer(ch chan string) {
	defer wg.Done()
	for data := range ch {
		fmt.Println(data)
	}
}

func provider(ch chan string) {
	defer wg.Done()
	for i := 0; i < 10; i++ {
		ch <- fmt.Sprintf("data: %v", i)
	}
	close(ch)
}

func main() {
	ch := make(chan string, 5)
	wg.Add(2)
	go consumer(ch)
	go provider(ch)
	wg.Wait()
	fmt.Println("main done")
}
