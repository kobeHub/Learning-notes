package main

// Test for a practical usage of interface
// different type use same method to implement inherit
import (
  "fmt"
)

type SalaryCalculator interface {
  CalculateSalary() int
}

type Permanent struct {
  empId string
  basicpay int
  pf int
}

type Contract struct {
  empId string
  basicpay int
}

//salary of permanent employee is sum of basic pay and pf
func (p Permanent) CalculateSalary() int {
  return p.basicpay + p.pf
}

// salary of contract is basic pay
func (c Contract) CalculateSalary() int {
  return c.basicpay
}

// calculate total employees' salary
func TotalSalary(emps []SalaryCalculator) int{
  expense := 0
  for _, v := range emps {
    expense += v.CalculateSalary()
  }
  return expense
}

func main() {
  p1 := Permanent {
    empId : "Td011",
    basicpay: 12000,
    pf: 2000,
  }
  c1 := Contract {
    empId: "Ts110",
    basicpay: 5000,
  }
  c2 := Contract {
    empId: "Ts111",
    basicpay: 6000,
  }

  var emps []SalaryCalculator = []SalaryCalculator{p1, c1, c2}
  expense := TotalSalary(emps)
  fmt.Println("All the employees")
  fmt.Println(emps)
  fmt.Println("Total salary:", expense)
}
