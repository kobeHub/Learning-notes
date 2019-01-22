# 接口

[TOC]

## 1. 接口定义

接口是面向对象编程中的概念，其基本含义是`接口定义了一个对象的基本行为`。一个接口定义了一个对象应该去做什么`what`，而具体行为如何实现就取决于对象本身`how`。 

在golang中，接口是一个方法签名的集合。当一个类型提供了对于接口中所有方法的定义，那么就可以说该类型实现了该接口，这与OOP的概念极为相似。也就是说接口确定了一个类型该有的所有方法，一个类型确定了如何实现这些方法。

## 2. 声明以及实现

声明一个接口的基本语法是

```go
type name interface {
    Method1() Type
    Method1() Type
    ...
}
```

对于已经声明的任何接口，可以使用定义在同一个包内的任何一个类型实现该接口的所有方法，此时该类型就是实现了该接口。

```go
type VowelsFinder interface {
  FindVowels() []rune
}

const VOWELS = "aeiouAEIOU"

// myString implement the VowelsFinder interface
func (s myString) FindVowels() []rune {
  var vowels []rune
  for _, char := range s {
    for _, target := range VOWELS {
      if char == target {
        vowels = append(vowels, char)
      }
    }
  }

  return vowels
}
```

以上代码定义了一个`VowelsFinder` 的接口，并且使用`string`类型的别名实现了该接口中的方法。`FindVowels（）方法的接收器是myString` 。对于一个接口的实现不需要借助类的概念，只需要是在包内良好定义的类型即可。但是根据方法的性质以及接口的性质，使以接口变量或者使用类型变量都可以对于接口中的方法进行调用。

### 2.1 一个典型的接口应用

接口一般用于实现可复用的面向对象的代码，通过使用接口可以实现继承的概念。

```go
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
```

如上述代码，存在两种员工，为了进行统一的工资计算，可以使用接口并且定义一个计算工资的方法，然后让两种类型分别实现该方法，所以可以通过将两种对象归一化为接口对象然后调用同一个方法`CalculateSalary（）`。而计算所有员工的工资时，就可以使用接口类型的切片作为参数，此时两种类型的员工都可以作为参数进行传递。

使用接口的最大的好处是，可以大大提升代码的可复用性，对于新添加的类型可以不需要改变`TotalSalary（）`的实现。

### 2.2 接口类型的内部表示

一个接口类型的内部表示可以用一个元组来表示，`（type，value）` type字段依赖于底层的具体的类型，value是具体类型的值。对于实现了一个特定接口的类型而言，类型与实际中的对象类型一致。

```go
package main


import (
  "fmt"
  "reflect"
)

type Tester interface {
  Test()
}

type double float64

func (i double) Test() {
  fmt.Printf("Interface type:%T %s, value %v\n", i, reflect.TypeOf(i), i)
}

func main() {
  i := double(12.90)
  var t Tester
  fmt.Printf("Tester: %T %v\n", t, t)
  t = i
  t.Test()
}
//
```

以上代码声明了一个接口并且使用`double`类型进行实现，对于一个没有具体实现的接口变量而言，其类型以及值都是`nil` ，而一个接口变量可以指向一个已经实现了该接口的具体的一个类型，此时该接口变量的类型以及值都变成了具体的指向的变量.

### 2.3 空接口

一个接口没有定义任何方法就是空接口，可以使用`interface{}`来定义一个空接口。由于空接口没有任何方法，所以任何一个类型都可以看做实现了该接口，也就是说一个空接口变量可以指向一个任意类型的变量。相当于面向对象中的`Object`基类。可以用作函数的参数用以接受任意类型的参数.

```go
func describe(i interface{}) {  
    fmt.Printf("Type = %T, value = %v\n", i, i)
}
```

 可以借助空接口实现类型的确认，对于一个空接口变量而言，可以使用`i.(Type)`的语法获取该变量对应于类型`Type` 的值。如果真是类型是type的化可以获得相应的值，如果不是，会返回一个panic错误。可以根据返回值来判断是否属于该类型。

```go
// use empty interface to assert int type
func assert(i interface{}) {
  v, ok := i.(int)
  if ok {
    fmt.Println(v, reflect.TypeOf(v))
  } else {
    panic("The type is not int!")
  }
}
```

### 2.4 类型比较

可以借助空接口实现类型的比较，当然接口之间也可以进行类型的比较。可以使用`switch...case`来进行类型的判断，需要获取一个变量的类型，可以使用`i.(type)` 的语法。

```go
// type switch
func findType(i interface{}) {
  // 如果需要调用Tester接口的方法，必须进行显式的类型转换，也就是获取Tester
  // 类型的值
  switch v := i.(type) {
  case string:
    fmt.Println("It is a string")
  case int:
    fmt.Println("It is an int")
  case Tester:
    v.Test()
  default:
    fmt.Println("Unknown")
  }
}
```

但是注意需要获取一个空接口变量真实类型的值后才可以调用该类型的方法。所以在`switch`后通常需要进行赋值操作