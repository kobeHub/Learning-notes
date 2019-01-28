# Method in golang

[TOC]

## 1. method 的作用

method通常作为一个特定类型的方法，用以对该类型进行操作，该类型可以是自定义的结构体也可以是预定义好的一些类型的`alias` .对于一个结构体的method的定义是在同一个包内完成的。对于method的一般语法是`func (T type) methodName (args Type) {}` ,第一个括号中的是该方法的接收器需要指定特定的类型，也就是定义一个属于某个类型的方法。

对于可以使用method实现的功能，也可以通过使用函数来实现。但是golang不是一个完全面向对象的语言，不支持类的结构，所以需要使用结构体进行替代，这是就需要方法来弥补类的操作。在golang中不允许函数的重载，也就是说不允许存在相同函数名的函数。但是可以使用方法，实现不同类型使用同一个方法名。

**不支持重载：**

```go
// Function overload test
func Area(c Circle) float64 {
  return math.Pi * c.radios * c.radios
}

func Area(r Rectangle) int {
  return r.width * r.length
}

c := Circle {1.0}
r := Rectangle {2, 3}
fmt.Println(Area(c), Area(r))

// # command-line-arguments
// ./method.go:41:6: Area redeclared in this block
//	previous declaration at ./method.go:37:21
// ./method.go:58:19: cannot use c (type Circle) as type Rectangle in argument to Area

```



## 2. Method 的基本用法

method的使用需要指明具体的接收器以及其类型，包含参数列表，返回值以及函数体。对于不同的类型，可以使用相同的方法名进行“overload”操作。

```go
// Method with same name
func (c Circle) Area () float64 {
  return math.Pi * c.radios * c.radios
}

func (r Rectangle) Area () int {
  return r.length * r.width
}
c := Circle {1.0}
r := Rectangle {2, 3}
fmt.Println("Methods with same name:\n", c.Area(), r.Area())

//Methods with same name:
 3.141592653589793 6
```

### 2.1指针接收器以及值接收器

对于method的接收器而言，可以使用按值传递，也可以使用按指针传递。主要区别在于接收器使用指针类型时，接受器内部的数据对于方法是可见的，也就是说可以通过该方法对于接收器的内部数据进行更改。如果使用值作为接收器时，此时相当于将原接收器进行了一次复制，所以在方法内部对于数据的更改对于调用者不可见。

```go

func (e Employee) IncreaseSalary () {
  fmt.Printf("%s salay: %f increase by value\n", e.name, e.salary)
  e.salary += 1000
}

func (e *Employee) IncreaseAge () {
  fmt.Printf("%s age: %d\n", e.name, e.age)
  e.age += 2
}
emp1.IncreaseSalary()
fmt.Println("Salary:", emp1.salary)
emp1.IncreaseAge()
fmt.Println("Age:", emp1.age)


Jack Chen salay: 6700.890000 increase by value
Salary: 6700.89
Jack Chen age: 60
Age: 62
```

在以上代码中，`IncreaseSalary（）`方法使用了按值传递的接收器，所以更改时不会对原值产生改变。而方法`IncreaseAge()` 使用了按指针传递的接收器，直接对于原数据进行更改。所以调用前者进行更改，对于用户是不可见的，后者对于用户可见。注意使用指针作为接收器时，可以使用`(&e).IncreaseAge()` 的语法，但是不够简洁，golang提供了可以直接使用value代替指针的语法。对于`e.IncreaseAge()` 在编译时会被解释为`(&e).IncreaseAge()` .

从另一方面来说，对于method的接收器来说，不管method的定义中是按值还是按指针传递，调用该方法时都可以使用两种方式。

### 2.2 使用指针接收器

指针1接收器主要用于在方法中对于接后器的改变需要在外部可见时。对与使用指针接收器，主要有以下两种情形。

+ 当复制一个数据结构需要较大的时空消耗时，可以使用指针接收器避免进行数据的复制
+ 在方法内部的更改，对于方法外部可见

###  2.3  匿名域中方法的使用

对于一个结构体如果用含有匿名域，而且该匿名域还定义了对应类型的方法，那么就可以直接使用最外层的结构体调用该匿名域的方法。

```go
type Address struct {
  city, province string
}

type Employee struct {
  name string
  age int
  salary float64
  Address
}

func (a *Address) changeCity (newCity string) {
  a.city = newCity
  fmt.Println("The new address:", a.city, a.province)
}
fmt.Println("\n\nNow emp move to Qingdao")
emp1.changeCity("Qingdao")
fmt.Println(emp1)
/*Now emp move to Qingdao
The new address: Qingdao Shandong
{Jack Chen 62 6700.89 {Qingdao Shandong}}
*/
```

## 3. 按值传递的接收器与参数按值传递

为了简化问题，对于一个参数按值传递的函数`value argument`，只可以接受value作为函数的参数。对于按值传递的接收器`value receiver` ，既可以使用值进行调用也可以使用指针进行调用。

```go
package main

import (  
    "fmt"
)

type rectangle struct {  
    length int
    width  int
}

func area(r rectangle) {  
    fmt.Printf("Area Function result: %d\n", (r.length * r.width))
}

func (r rectangle) area() {  
    fmt.Printf("Area Method result: %d\n", (r.length * r.width))
}

func main() {  
    r := rectangle{
        length: 10,
        width:  5,
    }
    area(r)
    r.area()

    p := &r
    /*
       compilation error, cannot use p (type *rectangle) as type rectangle 
       in argument to area  
    */
    //area(p)

    p.area()//calling value receiver with a pointer
}
```

由于golang是强类型的语言，所以对于一个函数的参数而言，必须使用对应的类型，按值传递的将指针进行传递会引起编译错诶。但是对于golang中的方法接收器，为了简便语法，可以直接忽略指针和值进行使用，对于接收器为值的方法，如果使用一个指针进行调用，编译器会自动进行翻译。`p.area()` --> `*p.area()` .

与之相似，如果一个接收器是指针的方法使用一个值进行调用，编译器也会进行自动的转化。

## 4. 非结构体上的方法

可以在一个非结构体上定义方法，但是必须遵守一定的规则。**为了在一个特定的类型上定义一个方法，对于接收器类型的定义以及方法的定义必须在同一个package中**。如果使用以下定义来实现整数的加法，编译器会报错

```go
package main

func (a int) add(b int) {  
}

func main() {

}
```

因为关于int类型的定义与该方法的定义不在同一个包中。如果需要对于一个内置的类型定义新的方法，可以使用内置类型的别名。

```go
package main

import "fmt"

type myInt int

func (a myInt) add(b myInt) myInt {  
    return a + b
}

func main() {  
    num1 := myInt(5)
    num2 := myInt(10)
    sum := num1.add(num2)
    fmt.Println("Sum is", sum)
}
```

