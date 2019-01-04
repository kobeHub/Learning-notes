package main

import (
  "fmt"
)

func pointer_test() {
  a := 123
  b := &a
  fmt.Println("The address of a:", b, a)
  fmt.Printf("The type of the address:%T\n", b)

  fmt.Println("nil pointer test")
  var ptr **int
  if ptr == nil {
    fmt.Println("ptr:", ptr)
    ptr = &b
    fmt.Printf("Address of b:%v, type:%T\n", ptr, ptr)
  }

  *b++;
  fmt.Println("a++:", *b, a)
}

func main() {
  pointer_test()
}

