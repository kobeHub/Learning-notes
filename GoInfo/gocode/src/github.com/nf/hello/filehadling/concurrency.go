package main
/*The usage of concurrency file process*/

import (
  "fmt"
  "os"
  "math/rand"
  "sync"
)

// the producer routine to produce random int
func producer(data chan int, wg *sync.WaitGroup) {
  n := rand.Intn(1000)
  data <- n
  wg.Done()
}

func consumer(data chan int, done chan bool) {
  f, err := os.Create("concurrent")
  if err != nil {
    fmt.Println(err)
    return
  }
  for d := range data {
    if _, err := fmt.Fprintln(f, d); err != nil {
      fmt.Println(err)
      f.Close()
      done <- false
      return
    }
  }
  err = f.Close()
  if err != nil {
    fmt.Println(err)
    done <- false
    return
  }
  done <- true
}

func main() {
  data := make(chan int)
  done := make(chan bool)
  wg := sync.WaitGroup{}
  for i := 0; i < 100; i++ {
    wg.Add(1)
    go producer(data, &wg)
  }
  go consumer(data, done)
  go func() {
    wg.Wait()
    close(data)
  }()
  if <- done {
    fmt.Println("file written successfully")
  } else {
    fmt.Println("file written failed")
  }

}
