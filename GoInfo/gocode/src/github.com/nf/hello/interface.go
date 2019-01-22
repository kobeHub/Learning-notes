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

// use an empty interface variable as argument to accept any
// type variables
func describe(i interface{}) {
  fmt.Printf("The type of variable: %T, value: %v\n", i, i)
}

// use empty interface to assert int type
func assert(i interface{}) {
  v, ok := i.(int)
  if ok {
    fmt.Println(v, reflect.TypeOf(v))
  } else {
    panic("The type is not int!")
  }
}

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


func main() {
  i := double(12.90)
  var t Tester
  fmt.Printf("Tester: %T %v\n", t, t)
  t = i
  t.Test()

  describe(12-90i)
  describe("Test")
  describe('a')
  describe(i)

  assert(10)
  //assert("test")

  findType(100)
  findType("dsfds")
  findType(i)
  findType(10.9)

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
}
