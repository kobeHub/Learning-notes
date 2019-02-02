package main

import (
  "fmt"
  "time"
)


// function to recover from main goroutines
func recoverName() {
  if r := recover(); r != nil {
    fmt.Println("recover from", r)
  }
}

func recovery() {
  if r := recover(); r != nil {
    fmt.Println("recovered from", r)
  }
}

// another goroutine recover
func a() {
  defer recovery()
  fmt.Println("Inside A")
  go b()
  time.Sleep(1 * time.Second)
}

func b() {
  fmt.Println("Inside B")
  panic("Panic from B")
}

func fullName(firstName, lastName *string) {
  defer recoverName()
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

  fmt.Println()
  a()
  fmt.Println("normally returned from main")
}
