# First class function

[TOC]

## 1. 第一类函数

一个支持第一类函数的语言，允许将一个函数赋值给一个变量，作为参数传递，作为函数的返回值，也就是将函数作为普通值进行处理。golang可以使用多种方式使用第一类函数。

### 1.1 匿名函数

可以将一个没有函数名的函数体赋值给一个变量，然后使用该变量调用该函数。称之为匿名函数。使用匿名函数时可以直接将其赋值给一个变量，然后使用该变量调用该函数，也可以直接在函数体后调用该函数，类似js的写法。

```go
var test = func() {
  fmt.Println("The function is anonymous function")
}

func main() {
  test()
  fmt.Printf("test type: %v, %T\n", test, test)
  func (s string) {
    fmt.Println("The anonymous function test", s)
  }("arguments")
}
/*The function is anonymous function
test type: 0x486eb0, func()
The anonymous function test arguments
*/
```

匿名函数的类型是`func()`,注意，将一个匿名函数赋值给一个变量时，可以将其赋值给另一个变量，这两个变量指向同一个地址。匿名函数没有具体名称。

### 1.2 用户定义的函数类型

可以使用函数类型作为一个新的类型，通过定义函数签名，可以将一类具有相同签名的函数作为一个集合，可以使用改函数变量来存放该值。

```go
package main

import (  
    "fmt"
)

type add func(a int, b int) int

func main() {  
    var a add = func(a int, b int) int {
        return a + b
    }
    s := a(5, 6)
    fmt.Println("Sum", s)
}
```

### 1.2 高阶函数

高阶函数是指使用了函数作为参数或者返回值的函数，这是firstclass函数的一种基础的使用方式：

```go
// high order function use functions as arguments
func simple(a, b int, f func(c, d int) int) {
  fmt.Println("result from function", f(a, b))
}

// high order functions use functions as return
func func_return(a int) func(int) int {
  f := func(tmp int) int {
    return a + tmp
  }
  return f
}
func main() {
  fmt.Println("Usage of high order functions")
  simple(1, 2, func(c, d int) int {
    return c * d
  })

  res := func_return(2)(1)
  fmt.Println("result from the returned function", res)
}
```

定义了两个高阶函数，分别使用函数作为参数，此时参数函数所需要使用的参数也都作为高阶函数的参数传入；使用函数作为分高阶函数的返回值，此时可以直接在其后加上括号放入所需要的参数进行调用。

## 2. 闭包

闭包是匿名函数的一个使用特例。闭包是指匿名函数访问了定义在函数外部的变量。每一个闭包具有自己的周边变量，闭包使用了这些周边变量。

```go
package main

import (  
    "fmt"
)

func appendStr() func(string) string {  
    t := "Hello"
    c := func(b string) string {
        t = t + " " + b
        return t
    }
    return c
}

func main() {  
    a := appendStr()
    b := appendStr()
    fmt.Println(a("World"))
    fmt.Println(b("Everyone"))

    fmt.Println(a("Gopher"))
    fmt.Println(b("!"))
}
```

在以上代码中，分别使用a，b两个变量作为两个实现相同的函数，每i一个函数具有自己独立的闭包。

## 3. 典型使用场景

first class function通常用于需要对于一批对象进行批处理，比如进行检索，进行函数的映射。使用这种函数可以得到更为简洁的代码。以下面的代码为例，需要检索所有满足参数函数的条件对象:

```go 
package main


import (
  "fmt"
)

type student struct {
  firstName string
  lastName string
  grade string
  country string
}


// filter all the students which make the `f` return true
func filter(s []student, f func(student) bool) []student {
  var res []student
  for _, v := range s {
    if f(v) {
      res = append(res, v)
    }
  }
  return res
}

// impl iMap
func iMap(s []*student, f func(*student)) {
  for _, v := range s {
    f(v)
  }
}

func main() {
  s1 := student {
    "Nikofl",
    "Inno",
    "B",
    "Japan",
  }
  s2 := student{
    "James",
    "Leborn",
    "A",
    "America",
  }
  s3 := student {
    "Kiturl",
    "Deropmerl",
    "B",
    "Greek",
  }

  students := []student{s1, s2, s3}
  set := filter(students, func(s student) bool {
    return s.grade == "B"
  })
  fmt.Println("All the students whose grade is B\n", set)

  sets := []*student{&s1, &s2, &s3}
  fmt.Println("Now every students' grade should be upgrade")
  iMap(sets, func(s *student) {
    (*s).grade += "+"
  })

  for _, v := range sets {
    fmt.Println(*v)
  }
}
/*All the students whose grade is B
 [{Nikofl Inno B Japan} {Kiturl Deropmerl B Greek}]
Now every students' grade should be upgrade
{Nikofl Inno B+ Japan}
{James Leborn A+ America}
{Kiturl Deropmerl B+ Greek}
*/
```





















