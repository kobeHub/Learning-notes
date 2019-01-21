package main

import (
  "fmt"
)

func main() {
  fmt.Println("Tests for inner package struct")

  var b Bike
  b.Maker = "BMW"
  b.model = "Tk-100"
  b.Price = 1200.5
  fmt.Printf("The bike %v, %T\n", b, b)
}
