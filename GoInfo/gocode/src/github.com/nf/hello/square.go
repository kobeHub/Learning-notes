package main

import (
  "fmt"
)

// go routine to compute cube
func ComCubes(num int, cubeop chan int) {
  sum := 0
  for num != 0 {
    digit := num % 10
    sum += digit * digit * digit
    num /= 10
  }
  cubeop <- sum
}

// go routine to compute square
func ComSquare(num int, squareop chan int) {
  sum := 0
  for num != 0 {
    digit := num % 10
    sum += digit * digit
    num /= 10
  }
  squareop <- sum
}

func main () {
  //test for compute square + cube
  num := 123
  squc := make(chan int)
  cubech := make(chan int)
  go ComCubes(num, cubech)
  go ComSquare(num, squc)

  squares, cubes := <-squc, <-cubech
  fmt.Println("Final output:", squares + cubes)
}
