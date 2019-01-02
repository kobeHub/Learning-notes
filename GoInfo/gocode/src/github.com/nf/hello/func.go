package main

import "fmt"

func Find(sets []int, targets ...int) {
  fmt.Printf("Type of the targets: %T\n", targets)
  fmt.Println("The datasets:")
  fmt.Println(sets)

  if len(targets) == 0 {
    fmt.Println("No target existed!")
    return
  }

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

func change(s ...string) {
  s[0] = "go"
  s = append(s, "play")
  fmt.Println(s)
}

func change_(s ...string) {
  s[0] = "Rust"
  c := make([]string, cap(s)*2)
  copy(c, s)
  c[2] = "test"
  s = c
  fmt.Println(s)
}

func main() {
  sets := []int{12, 23, 56, 765, 1e3, 90, 99}
  Find(sets)
  test := []string{"Hello", "World"}
  change_(test...)
  fmt.Println(test)
}
