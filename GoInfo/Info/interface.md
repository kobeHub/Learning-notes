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

## 3. 接口实现时的不同接收器

对于一个特定的类型，实现一个接口需要实现接口提供的所有方法，由于method可以使用值或者指针两种接收器。所以在使用接口变量调用方法时，因为接收器类型的不同会存在一定的差异。但是对于类型变量调用该方法时不存在接收器的差异。

**在使用接口变量时：**

+ 如果接收器是值`value`，那么可以使用`value， pointer`向接收器变量赋值，然后进行调用
+ 如果接收器是指针那么只可以使用`pointer`向接收器变量赋值，不可以使用`value`赋值

```go
/* Pointer receiver and value receiver*/
type Person struct {
  name string
  age int
}

type Address struct {
  city, state string
}

func (p Person) Test() {
  fmt.Println("Value receiver person", p.name, "age:", p.age)
}

func (a *Address) Test() {
  fmt.Println("Pointer recevier address:", a.city, a.state)
}


fmt.Println("\nTest for value reciver and pointer receiver")
  var d1 Tester
  p1 := Person {"Jack", 35}
  p2 := Person {"Jordan", 66}
  d1 = p1
  d1.Test()
  d1 = &p2
  d1.Test()

  a1 := Address {"LosAngle", "California"}
  d1 = &a1
  d1.Test()
//  d1 = a1    // 不可以使用该赋值，因为实现了该接口的是指针类型
  a1.Test()    // 对于方法而言，不管接收器类型是指针还是值都可以
               // 使用任何一种形式调用，编译器会自动进行编译
  (&a1).Test()


/*Test for value reciver and pointer receiver
Value receiver person Jack age: 35
Value receiver person Jordan age: 66
Pointer recevier address: LosAngle California
Pointer recevier address: LosAngle California
Pointer recevier address: LosAngle California
*/
```

如上述代码，对于类型变量`a1`可以使用指针类型或者使用值形式调用接扩中定义的方法，使用指针时编译器会自动为指针进行解引用操作`dereference`。

对于使用value的实现接口的类型，可以使用value或者pointer向接口变量赋值。但是使用pointer接收器的类型，只可以使用pointer向接口赋值。因为`Tester`接口是`*Address`实现的，`Address`  没有进行实现。**原因是因为可以使用value调用的方法必须有一个指向该值的指针或者说可以取得该值的地址**，但是对于存储在接口变量中的具体指不可以取得其地址，因此也就不可以直接进行调用，也就无法完成赋值。

## 4. 高级用法

### 4.1 实现多个接口

一个type可以实现多个接口，分别实现不同的功能。

```go
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
```

output：

```go
All the employees
[{Td011 12000 2000} {Ts110 5000} {Ts111 6000}]
Total salary: 25000
Now descrease Td011 salry 100
Permanent:Td011 salary:13900
Now increase Td011 salry 1000
Permanent:Td011 salary:14900
```

在以上代码中，两种type实现了两个接口，对于`SalaryChanger`接口中方法的实现，使用的是指针类型的接收器，对于`SalaryCalculator`接口的实现，使用的是value的接收器。并且定义了统一的接口`SalaryOption`作为两个接口的逻辑“父接口”，可以使用该变量调用所有操作。

### 4.2 接口嵌套

尽管golang中没有类的概念，也不提供继承机制，但是可以通过接口实现相似的功能，可以使用接口嵌套实现。