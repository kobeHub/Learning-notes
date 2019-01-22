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
}
