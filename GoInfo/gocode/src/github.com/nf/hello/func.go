package main

import "fmt"

func Find(sets []int, targets ...int) {
  fmt.Printf("Type of the targets: %T\n", targets)
  fmt.Println("The datasets:")
  fmt.Println(sets)

  var found bool

  for _, v := range targets {
    found = false
    for j, va := range sets {
      if v == va {
        found = true
        fmt.Println("Found", v, "in the sets index:", j)
      }
    }
    if !found {
      fmt.Println("Not found", v, "in the sets")
    }
  }
}

func main() {
  sets := []int{12, 23, 56, 765, 1e3, 90, 99}
  Find(sets, 90)
  Find(sets, 90, -12, 77, 12, 23)
}
