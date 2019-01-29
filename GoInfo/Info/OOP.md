# Object-oriented Programming

[TOC]

## 1. Go不是纯面向对象的语言

Golang不是一个纯面向对象的语言，摘自Golang [FAQ](This excerpt taken from Go's [FAQs](https://golang.org/doc/faq#Is_Go_an_object-oriented_language) answers the question of whether Go is Object Oriented.) ,解释了golang是否为面向对象的语言。

```
Yes and no. Although Go has types and methods and allows an object-oriented style of programming, there is no type hierarchy. The concept of “interface” in Go provides a different approach that we believe is easy to use and in some ways more general. There are also ways to embed types in other types to provide something analogous—but not identical—to subclassing. Moreover, methods in Go are more general than in C++ or Java: they can be defined for any sort of data, even built-in types such as plain, “unboxed” integers. They are not restricted to structs (classes).  
```

也就是说使用类型以及menthods可以实现面向对象编程的类似功能，但是不存在类型的继承。使用了`interface`实现了不一样的解决方案，更加易用而且更加通用。

可以使用`struct`代替类,如果需要使用可见性约束的类，可以定义包内类型，然后定义`exported methods` ，定义`New() T`作为构造器的模拟，同时使用其他方法，使得接收器为该类型变量：

```go
// Define the struct with all fields exported
type employee struct {
  firstName, lastName string
  totalLeaves, leavesTaken int
}

// Exported method with a receiver of Employee
func (e employee) LeavesRemaining() {
  fmt.Printf("%s %s has %d leaves remaining.\n",
              e.firstName, e.lastName, (e.totalLeaves - e.leavesTaken))
}

// New function as a Constructor
func New(firstName, lastName string, totalLeaves, leavesTaken int)  employee {
  e := employee {firstName, lastName, totalLeaves, leavesTaken}
  return e
}

// Property of employee
func (e employee) GetFirstName() string {
  return e.firstName
}

func (e employee) GetLastName() string {
  return e.lastName
}

func (e employee) GetTotal() int {
  return e.totalLeaves
}
```

## 2. 使用组合代替继承

Golang不支持继承机制，但是支持组合`composition`。组合的一般定义是能将多个部分放在一起。在Golang中使用接阔的嵌套实现组合机制，

### 2.1 接口的嵌套

