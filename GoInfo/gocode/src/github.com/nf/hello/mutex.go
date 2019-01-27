package main

/*Simple case to use mutex*/

import (
  "fmt"
  "sync"
)

var x = 1

// Avoid race condition with Mutex
func increase(wg *sync.WaitGroup, m *sync.Mutex) {
  m.Lock()
  x += 1
  m.Unlock()
  wg.Done()
}

// Avoid race condition with channel
func increment(wg *sync.WaitGroup, ch chan bool) {
  ch <- true    // ch is blocked and none routine can process go on
  x += 1
  <-ch
  wg.Done()
}

func main() {
  var wg sync.WaitGroup
  //var m sync.Mutex    // Use the same mutex to void race condition
  ch := make(chan bool, 1)
  for i := 0; i < 1000; i++{
    wg.Add(1)
    //go increase(&wg, &m)
    go increment(&wg, ch)
  }
  wg.Wait()
  fmt.Println("Final result:", x)
}
