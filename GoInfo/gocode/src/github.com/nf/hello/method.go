package main

/*有关方法的测试，包含结构体作为接收器的测试，以及基础类型的测试*/

import (
  "fmt"
  "math"
)


type INT int

func (i INT) add(j INT) INT {
  return i + j
}

type Address struct {
  city, province string
}

type Employee struct {
  name string
  age int
  salary float64
  Address
}

type Circle struct {
  radios float64
}

type Rectangle struct {
  length, width int
}

// Define a method which receiver is Employee
func (emp Employee) showEmployee () {
  fmt.Printf("Employee %s, age %d, salary %f, city %s, province %s\n",
              emp.name, emp.age, emp.salary, emp.city, emp.province)
}


/*
// Function overload test
func Area(c Circle) float64 {
  return math.Pi * c.radios * c.radios
}

func Area(r Rectangle) int {
  return r.width * r.length
}
*/

// Method with same name
func (c Circle) Area () float64 {
  return math.Pi * c.radios * c.radios
}

func (r Rectangle) Area () int {
  return r.length * r.width
}

func (e Employee) IncreaseSalary () {
  fmt.Printf("%s salay: %f increase by value\n", e.name, e.salary)
  e.salary += 1000
}

func (e *Employee) IncreaseAge () {
  fmt.Printf("%s age: %d\n", e.name, e.age)
  e.age += 2
}

func (a *Address) changeCity (newCity string) {
  a.city = newCity
  fmt.Println("The new address:", a.city, a.province)
}

func main() {
  emp1 := Employee{
    name: "Jack Chen",
    age: 60,
    salary: 6700.89,
    Address: Address {
      city: "Jinan",
      province: "Shandong",
    },
  }

  emp1.showEmployee()
  c := Circle {1.0}
  r := Rectangle {2, 3}
  fmt.Println("Methods with same name:\n", c.Area(), r.Area())

  emp1.IncreaseSalary()
  fmt.Println("Salary:", emp1.salary)
  emp1.IncreaseAge()
  fmt.Println("Age:", emp1.age)

  fmt.Println("\n\nNow emp move to Qingdao")
  emp1.changeCity("Qingdao")
  fmt.Println(emp1)


  fmt.Println("Built in type method:")
  i, j := INT(12), INT(55)
  fmt.Println(i, j, i.add(j))
}
