package main

import (
  "fmt"
)

// extract common digit operation from the two function
func DigitOps(num int, digitch chan int) {
  for num != 0 {
    digit := num % 10
    num /= 10
    digitch <- digit
  }
  close(digitch)
}

// go routine to compute cube
func ComCubes(num int, cubeop chan int) {
  sum := 0
  digitch := make(chan int)
  go DigitOps(num, digitch)

  for digit := range digitch {
    sum += digit * digit * digit
  }
  cubeop <- sum
}

// go routine to compute square
func ComSquare(num int, squareop chan int) {
  sum := 0
  sch := make(chan int)
  go DigitOps(num, sch)

  for d := range sch {
    sum += d * d
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
