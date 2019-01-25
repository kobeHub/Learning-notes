/*进行单向channel测试，以及单向channel与双向channel
隐式转换的测试*/
package main

import "fmt"

func sendData(sendch chan<- int) {
  sendch <- 10
  fmt.Printf("Inner routine: %T\n", sendch)
}

func main() {
  chanl := make(chan int)
  go sendData(chanl)
  fmt.Printf("%v, %T\n", <-chanl, chanl)
}
