package main

import "fmt"

type Employee struct {
  firstName, secondName string
  age, salary int
}

func test() {
  emp1 := Employee {
    firstName: "Sam",
    secondName: "Bryant",
    age: 23,
    salary: 5000,
  }

  emp2 := Employee{"Tomoas", "Paul", 29, 20000}
  fmt.Println("Employee 1", emp1)
  fmt.Println("Employee 2", emp2)
}

func main() {
  test()
}
