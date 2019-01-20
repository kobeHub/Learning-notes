# Go struct

[TOC]

## 1. 结构体的组成

在golang中结构体与c中的相似，是一些域的集合，适用于将多个数据域作为同一个抽象概念的场景。结构体的定义以及声明需要使用大括号。使用关键字`type` 定义一个结构体，结构体中的一个每一个属性需要明确定义其名称，多个相同类型的属性可以写在一行并且使用逗号隔开。

也可以不使用关键字`type` 定义结构体，可以使用`var`定义一个匿名结构体。没有新定义一个类型。

### 1.1 声明一个结构体

```go
type People struct {
    firstName string
    lastName string
    age int
}

var People = struct {
    firstName, lastName string
    age int   //不需要逗号，因为类型后面不会自动补;
}{
    "Jack",
    "Ma",
    60,      //需要逗号，否则会自动填充分号
}
```

声明一个匿名结构体时，需要定义结构体的属性不需要指明结构体的名称，即没有给该结构体声明一个特定的类型。获取其类型时会输出`struct {}` 的定义格式。

```go
var people = struct {
  firstName, lastName string
  age int
}{
  "Jack",
  "Ma",
  60,
}


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
// Output:
// Employee 1, {Sam Bryant 23 5000}, main.Employee, main.Employee
// Employee 2, {Tomoas Paul 29 20000} struct
// People: {Jack Ma 60} struct { firstName string; lastName string; age int }
// Normal struct and anonymous struct type diff:
// Anonymous struct: struct { firstName string; lastName string; age int }
// Struct: main.Employee

```

### 2. 结构体的访问

golang中结构体的访问与c的一致，相当于`public` 的类，可以直接对于一个结构体变量中的任何一个属性进行访问赋值。同时可以声明一个没有初始值的结构体其所有的属性值将会按照属性类型的默认值进行赋值。

如下代码，对于`string`类型赋予默认值`""` ,对于int类型赋予0

```go
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

// Test for accessing to elements of struct:
// Employee 3: {John James 0 0} Employee 4: {  0 0}
// Now assign to emp3 and emp4
// Employee 3: {Test James 30 30000} Employee 4: {Mike Don 23 2000}
```

### 1.3 结构体的比较

结构体可以使用`==`进行判断两个相同类型的结构体的值是否相等。但是需要注意，定义的结构体的每一个属性必须是可以比较的。

```go

func equal() {
  emp1 := Employee{"Mike", "Tang", 66, 9000}
  emp2 := Employee{"Mike", "Tang", 66, 90}

  if emp1 == emp2 {
    fmt.Println("The same employee")
  } else {
    fmt.Println("Not one")
  }

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
  }
}

// ./struct_.go:132:13: invalid operation: image1 == image2 (struct containing map[int]int cannot be compared)
```



## 2. 匿名与可导出

### 2.1 匿名域

对于结构体可以进行匿名声明，不需要指明其具体类型。对于结构体中的域也可以指明其名称只需要指明其类型，即为匿名域，匿名属性。匿名属性的访问可以使用`variable.type` 的格式。指明结构体变量的某一个类型的属性，即可以进行访问。但是注意此时匿名域的类型必须不同，否则会报错`ambigous selector ` 

```go
  fmt.Println("\n\n")
  type People struct {
    string
    int
  }
  p1 := People{"Michale", 50}
  fmt.Println(p1, p1.string, p1.int)
```

### 2.2 嵌套结构体

结构体可以进行嵌套，因为对于一个结构体而言，其每一个域都可以是任意类型。一个结构体可以作为另一个结构体的属性进行嵌套。对于结构体内部的结构体的属性，需要使用两次`.`才可以进行访问。

```go
package main

import (  
    "fmt"
)

type Address struct {  
    city, state string
}
type Person struct {  
    name string
    age int
    address Address
}

func main() {  
    var p Person
    p.name = "Naveen"
    p.age = 50
    p.address = Address {
        city: "Chicago",
        state: "Illinois",
    }
    fmt.Println("Name:", p.name)
    fmt.Println("Age:",p.age)
    fmt.Println("City:",p.address.city)
    fmt.Println("State:",p.address.state)
}
```

### 2.3 结构体提升域

对于嵌套结构体而言，为了直接访问嵌套在内部的结构体，可以使用匿名域作为内部结构体的域。此时内部结构体的所有属性的能访问都进行了提升，可以使用外部变量进行直接访问。而不需要使用两次点操作符。

```go
type People struct {
  name string
  age int
  Address    // Promoted field, can be access by variable directly
}

type Address struct {
  city, street string
}

var p People
p.name = "Jackon"
p.age = 30
p.Address = Address {
    city: "Beijing",
    street: "二环",
}
fmt.Println(p, p.Address.city, p.street)

```

### 2.4 可导出的结构体以及可见域

与golang中的包管理机制一致，为了可以在其他包访问定义的结构体，可以使用大写字母作为定义类型的首字母。对于结构体中的属性，其可见性也按照这种机制，包外访问的只能是可导出结构体的首字母大写的属性.

```go
import "github.com/nf/computer"

func exported() {
  var spec computer.Spec
  spec.Maker = "Apple"
  spec.model = "fdsfsd"   // 包外不可见属性
  spec.Price = 5000.80
  fmt.Println(spec)
}
// # command-line-arguments
// ./struct_.go:106:7: spec.model undefined (cannot refer to unexported field or method model)
```



