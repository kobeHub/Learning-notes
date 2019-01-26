package main


import (
  "fmt"
  "time"
)

func Hello(a chan bool) {
  fmt.Println("In the Hello function sleep")
  time.Sleep(3 * time.Second)
  fmt.Println("hello go routine awake!")
  a <- true
}

func main() {
  fmt.Println("Main initialized!")

  a := make(chan bool)
  go Hello(a)

  if <-a {
    fmt.Println("The hello run successfully")
  } else {
    fmt.Println("Hello error")
  }

  fmt.Printf("%v, %T; %v, %T; %v, %T\n", time.Hour, time.Hour,
              time.Minute, time.Minute, time.Second, time.Second)

  var test = make(chan int, 20)
  fmt.Printf("The type of channel:%T, len:%v, cap:%v\n", test, len(test), cap(test))
  test <- 122
  test <- 11
  fmt.Printf("After changed, Len:%v, cap:%v\n", len(test), cap(test))

}
