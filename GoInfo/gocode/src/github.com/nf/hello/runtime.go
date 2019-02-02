package main

import "fmt"
import "runtime/debug"


func outOfBound() {
  defer recovery()
  a := []int{1, 2, 0}
  fmt.Println(a, a[1]/a[2])
  fmt.Println("normally returned from outOfBound")
}

func recovery() {
  if r := recover(); r != nil {
    fmt.Println("recover successfully")
    debug.PrintStack()
  }
}

func main() {
  outOfBound()
  fmt.Println("normally returned from main")
}
