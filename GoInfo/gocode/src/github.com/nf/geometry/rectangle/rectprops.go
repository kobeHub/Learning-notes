// Define rectangle behaviour
package rectangle

import "math"

func init() {
  println("Geomotry/rectangle package initialized")
}

func Area(length, width float64) float64 {
  return length * width
}

func Diagonal(length, width float64) float64 {
  diagonal := math.Sqrt(length * length + width * width)
  return diagonal
}
