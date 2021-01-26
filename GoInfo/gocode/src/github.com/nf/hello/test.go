package main

import (
  "fmt"
  "unsafe"
  "math"
)

var (
  basic = 1 + 2i

)

func rectTest(length, width float64) (area, per, div float64) {
  area = length * width
  per = (length + width) / 2
  div = math.Sqrt(length*length + width*width)
  return
}

func main() {
  a, b := 'a', 12
  c, d, e := 123, "Test", "go"

  fmt.Println(5/2)
  fmt.Println(5 / 2.0)
	fmt.Println(a, b, c, d, e)
  fmt.Println("Type test:")
  fmt.Printf("a: %T, size: %d, b: %T, size: %d", a, unsafe.Sizeof(a), b, unsafe.Sizeof(b))

  c1 := 3 - 12i
  c2 := complex(1, 3)
  fmt.Println("Complex:", basic, c1, c2)
  fmt.Println("Add:", basic + c1, "Mul:", c1 * basic)

  fmt.Println("\nTest for func:")
  area, _, __ := rectTest(3, 4)
  fmt.Println("Area:", area, "div:", __, )
}
