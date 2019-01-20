package main

import (
  "fmt"
  "reflect"
  "github.com/nf/computer"
)

type Employee struct {
  firstName, secondName string
  age, salary int
}

var people = struct {
  firstName, lastName string
  age int
}{
  "Jack",
  "Ma",
  60,
}

type People struct {
  name string
  age int
  Address    // Promoted field, can be access by variable directly
}

type Address struct {
  city, street string
}

func test() {
  emp1 := Employee {
    firstName: "Sam",
    secondName: "Bryant",
    age: 23,
    salary: 5000,
  }

  emp2 := Employee{"Tomoas", "Paul", 29, 20000}
  fmt.Printf("Employee 1, %v, %s, %T\n", emp1, reflect.TypeOf(emp1), emp1)
  fmt.Println("Employee 2,", emp2, reflect.ValueOf(emp2).Kind())

  fmt.Println("People:", people, reflect.TypeOf(people))
  fmt.Println("Normal struct and anonymous struct type diff:")
  fmt.Printf("Anonymous struct: %T\n", people)
  fmt.Printf("Struct: %T\n", emp1)
}

func access() {
  fmt.Println("\n\nTest for accessing to elements of struct:")
  emp1 := Employee{firstName: "John", secondName: "James"}
  var emp2 Employee
  fmt.Println("Employee 3:", emp1, "Employee 4:", emp2)

  fmt.Println("Now assign to emp3 and emp4")
  emp2.firstName = "Mike"
  emp2.secondName = "Don"
  emp2.age = 23
  emp2.salary = 2000
  emp1.firstName = "Test"
  emp1.age = 30
  emp1.salary = 30000
  fmt.Println("Employee 3:", emp1, "Employee 4:", emp2)
}

func EnhanceSalary(emp *Employee) *Employee {
  fmt.Println("Noe enhance the salary of", emp.firstName)
  fmt.Println((*emp).salary)
  emp.salary += 1000
  return emp
}

func Enhance(emp Employee) {
  emp.age += 100
}

func AccessAnon() {
  // 访问结构体的匿名域
  fmt.Println("\n\n")
  type People struct {
    string
    int
  }
  p1 := People{"Michale", 50}
  fmt.Println(p1, p1.string, p1.int)
}

// 嵌套结构体使用
func nested() {
  var p People
  p.name = "Jackon"
  p.age = 30
  p.Address = Address {
    city: "Beijing",
    street: "二环",
  }
  fmt.Println(p, p.Address.city, p.street)
}

// 可导入结构体的测试
func exported() {
  var spec computer.Spec
  spec.Maker = "Apple"
//  spec.model = "fdsfsd"
  spec.Price = 5000.80
  fmt.Println(spec)
}

func equal() {
  emp1 := Employee{"Mike", "Tang", 66, 9000}
  emp2 := Employee{"Mike", "Tang", 66, 9000}

  if emp1 == emp2 {
    fmt.Println("The same employee")
  } else {
    fmt.Println("Not one")
  }
/*
  type image struct {
    data map[int]int
  }

  image1 := image{data: map[int]int{
    0: 188,
  }}
  image2 := image{data: map[int]int{
    0:188,
  }}

  if image1 == image2 {
    fmt.Println("Same image")
  } else {
    fmt.Println("not one")
  }*/
}

func main() {
  test()
  access()
  emp := Employee{"Jack", "Cheung", 23, 6000}
  var emp_ *Employee
  emp_ = EnhanceSalary(&emp)
  fmt.Println(*emp_)
  Enhance(emp)    // golang中的参数按值传递，传递的参数相当于重新开辟了一块内存空间
  fmt.Println(emp)
  AccessAnon()

  nested()
  exported()
  equal()
}
