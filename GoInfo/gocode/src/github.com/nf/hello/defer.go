package main

/*Simple case of defer usage*/
import (
	"fmt"
)

type person struct {
	name string
	age  int
}

func (p person) display() {
	fmt.Println(p.name, p.age)
}

func defer_test() {
	p := person{"Leborn James", 36}
	defer p.display()
	p.age += 1
	fmt.Println("Let me intro to u:")
	fmt.Println("and after one year his age:", p.age)
}

func printA(a *int) {
	fmt.Println("value of a deferred function", *a)
}

func printSlice(s []int) {
	fmt.Println("The slice in the defered function:")
	fmt.Println(s)
}

func printint(a int) {
	fmt.Println("Stack:", a)
}

func stack() {
	for i := 0; i < 5; i++ {
		defer printint(i)
	}
}

func main() {
	defer_test()
	a := 1
	defer printA(&a)
	a = 12
	defer printA(&a)
	a += 1
	fmt.Println("value in main function", a)

	s := []int{1, 12, 4, 5}
	defer printSlice(s)
	s = append(s, 122) //use append so the pointer changed, and direct to different address

	sa := make([]int, 3)
	defer printSlice(sa)
	sa[0] = 111
	sa[2] = 222
	stack()
}
