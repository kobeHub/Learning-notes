package main

import (
  "fmt"
)

var test = func() {
  fmt.Println("The function is anonymous function")
}

// user defined function types
type add func(a int, b int) int

// high order function use functions as arguments
func simple(a, b int, f func(c, d int) int) {
  fmt.Println("result from function", f(a, b))
}

// high order functions use functions as return
func func_return(a int) func(int) int {
  f := func(tmp int) int {
    return a + tmp
  }
  return f
}

// closures usage, return a function
func appendStr() func(string) string {
  var pre = "Hello"
  c := func(s string) string {
    pre += " " + s
    return pre
  }
  return c
}

func main() {
  test()
  t := test
  fmt.Printf("test type: %v, %T, %v, %T\n", test, test, t, t)
  func (s string) {
    fmt.Println("The anonymous function test", s)
  }("arguments")

  var a add = func(a, b int) int {
    return a + b
  }

  s := a(5, 7)
  fmt.Println("Sum", s)

  fmt.Println("Usage of high order functions")
  simple(1, 2, func(c, d int) int {
    return c * d
  })

  res := func_return(2)(1)
  fmt.Println("result from the returned function", res)

  fmt.Println("\nClosures cases:")
  str1 := appendStr()
  str2 := appendStr()
  fmt.Println(str1("world"))
  fmt.Println(str2("Everyone"))

  fmt.Println(str1("Gopher"))
  fmt.Println(str2("!"))
}
