package main

import (
	"fmt"
)

func fibo() func() int {
	res := []int{0, 0}
	return func() int {
		if res[1] == 0 {
			res[1] = 1
			return 0
		}
		res[0], res[1] = res[1], res[0] + res[1]
		return res[0]
	}
}

func main() {
	fmt.Println("Using closure version fibo")
	var fiboFunc func() int = fibo()
	for i := 0; i < 10; i++ {
		fmt.Print(fiboFunc(), " ")
	}
}
