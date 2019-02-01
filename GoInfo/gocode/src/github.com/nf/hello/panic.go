package main

import (
  "fmt"
)


func fullName(firstName, lastName *string) {
  defer fmt.Println("defered call in the fullName")
  if firstName == nil {
    panic("runtime error: the firstName is nil")
  }
  if lastName == nil {
    panic("runtime error: the lastName is nil")
  }
  fmt.Printf("%s %s\n", *firstName, *lastName)
  fmt.Println("normally returned from fullName")
}

func main() {
  name := "Hordan"
  defer fmt.Println("defered call from main routine")
  fullName(&name, nil)
  fmt.Println("normally returned from main")
}
