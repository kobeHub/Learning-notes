package main

// Test for a practical usage of interface
// different type use same method to implement inherit
import (
  "fmt"
)

type SalaryCalculator interface {
  CalculateSalary() int
  Display()
}

type SalaryChanger interface {
  IncreaseBaisc(ins int)
  DescreaseBasic(des int)
}


// 实现了多个接口，为了可以调用所有接口中的方法，需要定义一个总的接口
type SalaryOptions interface {
  SalaryCalculator
  SalaryChanger
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

func (p Permanent) Display() {
  fmt.Printf("Permanent:%s salary:%d\n", p.empId, p.basicpay + p.pf)
}

func (p *Permanent) IncreaseBaisc(ins int) {
  fmt.Printf("Now increase %s salry %d\n", p.empId, ins)
  p.basicpay += ins
}

func (p *Permanent) DescreaseBasic(des int) {
  fmt.Printf("Now descrease %s salry %d\n", p.empId, des)
  p.basicpay -= des
}

// salary of contract is basic pay
func (c Contract) CalculateSalary() int {
  return c.basicpay
}

func (c Contract) Display() {
  fmt.Println("Contract:%s salary:%d\n", c.empId, c.basicpay)
}

func (c *Contract) IncreaseBaisc(ins int) {
  fmt.Printf("Now increase %s salry %d\n", c.empId, ins)
  c.basicpay += ins
}

func (c *Contract) DescreaseBasic(des int) {
  fmt.Printf("Now descrease %s salry %d\n", c.empId, des)
  c.basicpay -= des
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

  var emp SalaryOptions = &p1    // 此处必须使用地址进行赋值
  // 因为需要更改对于调用者可见，所以method只可以使用指针接收器
  // 而实现该接口时使用指针接收器，所以实现该接口的是指针类型
  // 只有指针类型才可以进行赋值，然后再调用其他操作
  emp.DescreaseBasic(100)
  emp.Display()    // 接收器为value，可以使用指针类型的接口进行调用，编译器自动解引用
  emp.IncreaseBaisc(1000)
  emp.Display()

}
