// Geometry test
package main

import (
  "fmt"
  "github.com/nf/geometry/rectangle"
  "log"
)

var (
  rectLen float64 = 6
  rectWid float64 = 8
)

func init() {
  if rectLen < 0 {
    log.Fatal("length is less than 0")
  }
  if rectWid < 0 {
    log.Fatal("width is less than 0")
  }
  println("Geometry main package initialized")
}

func main() {
  fmt.Println("Gemetrical shape preperties:")
  fmt.Printf("area of rectangle: %.2f\n", rectangle.Area(rectLen, rectWid))
  fmt.Printf("diagonal of the rectangle: %.2f\n", rectangle.Diagonal(rectLen, rectWid))
}
