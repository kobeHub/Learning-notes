/*进行单向channel测试，以及单向channel与双向channel
隐式转换的测试*/
package main

import (
  "fmt"
  "time"
)

func sendData(sendch chan<- int) {
  sendch <- 10
  fmt.Printf("Inner routine: %T\n", sendch)
}

// Write into buffered channel
func write(ch chan int) {
  for i :=0; i < 5; i++ {
    ch <- i
    fmt.Println("Writer into channel", i)
  }
  close(ch)
}

// read from buffered channel every 1s
func read(ch chan int, mut chan bool) {
  time.Sleep(2 * time.Second)
  for i := range ch {
    fmt.Println("read value", i, "from chan")
    time.Sleep(1 * time.Second)
  }
  mut <- true
}

func main() {
  chanl := make(chan int)
  go sendData(chanl)
  fmt.Printf("%v, %T\n", <-chanl, chanl)

  ch := make(chan int, 2)
  mut := make(chan bool)
  go write(ch)
  go read(ch, mut)
  <-mut
}
