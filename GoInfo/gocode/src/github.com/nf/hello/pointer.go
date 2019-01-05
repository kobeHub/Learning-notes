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

func change_value(ptr *int) {
  *ptr = 123
}

func main() {
  pointer_test()
  var test = 12
  change_value(&test)
  fmt.Println("12 after change:", test)
}

