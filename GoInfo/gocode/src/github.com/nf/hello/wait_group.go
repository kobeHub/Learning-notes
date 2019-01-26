/*Simple program to use WaitGroup*/
package main

import (
  "fmt"
  "sync"
  "time"
)

// The routine spawned from main routine
func process(i int, wg *sync.WaitGroup) {
  fmt.Println("started Goroutine", i)
  time.Sleep(1 * time.Second)
  fmt.Println("Goroutine", i, "done.")
  wg.Done()
}

func main() {
  no := 5
  var wg sync.WaitGroup
  for i := 0; i < no; i++ {
    wg.Add(1)
    go process(i, &wg)
  }
  wg.Wait()
  fmt.Println("All the routines finished")
}

