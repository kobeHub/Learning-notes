package main

import (
  "fmt"
)

func main() {
  const MAX = int32(^uint32(0) >> 1)
  const MIN = ^MAX
  fmt.Println(MAX, MIN, int(MAX), int(MIN))
}
