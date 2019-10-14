package main

import (
	"fmt"
)

func f1() int {
	x := 5
	defer func() {
		x++
	}()
	return x
}

func f2(x int) int {
	defer func(x int) {
		x++
	}(x+1)
	return x
}

func f3() (x int) {
	x = 5
	defer func() {
		x++
	}()
	return x
}

func f4() (x int) {
	x = 5
	defer func(x int) {
		x++
	}(x)
	return x
}

func main() {
	fmt.Println(f1())
	fmt.Println(f2(5))
	fmt.Println(f3())
	fmt.Println(f4())
}


