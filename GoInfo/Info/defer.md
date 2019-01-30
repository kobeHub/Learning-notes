# defer usage

[TOC]

## 1.  基础用法 

`defer`语句主要用改变程序的执行流程，用于添加一个方法或者函数在一个方法执行结束返回之前调用defer指向的方法。

```go
type person struct {
  name string
  age int
}

func (p person) display() {
  fmt.Println(p.name, p.age)
}

func defer_test() {
  p := person{"Leborn James", 36}
  defer p.display()
  p.age += 1
  fmt.Println("Let me intro to u:")
  fmt.Println("and after onr year his age:", p.age)
}

func main() {
  defer_test()
}
/*Let me intro to u:
and after onr year his age: 37
Leborn James 36
*/
```

如上述代码所示，在函数`defer_test`中使用`defer`语句，调用了`person`的一个方法，在该函数返回之前，调用了person的display方法。

### 1.1 参数的值

需要注意的是，如果`defer`语句使用的参数为变量时，那么参数的值绑定的是使用该语句时的值，如果变量值发生改变，参数仍保留原始值。

```go
package main

import (  
    "fmt"
)

func printA(a int) {  
    fmt.Println("value of a in deferred function", a)
}
func main() {  
    a := 5
    defer printA(a)
    a = 10
    fmt.Println("value of a before deferred function call", a)

}

// value of a in deferred function 5  
// 
```

使用defer语句后，参数保留的是此时a的值，之后改变了其值，但是该函数使用的是按值传递，。只是把变量a的值复制了一遍，所以改变了原始a的值不对函数调用产生影响。**但是注意如果使用指针类型或者使用切片类型作为参数时，defer参数与main Function中的变量指向同一个地址。**

重点在于参数与原始的变量是否指向同一个地址。

```go

```

### 1.2 defers stack

当一个函数中使用多个defer调用，那么这些调用的执行顺序按照栈的形式，后进先出(LIFO)。也就是最后一个`defer`语句先执行，然后向上返回。所以一组defer调用也具备了栈的性质，所以可以实现倒序等操作。

```go
func main() {  
    name := "Naveen"
    fmt.Printf("Orignal String: %s\n", string(name))
    fmt.Printf("Reversed String: ")
    for _, v := range []rune(name) {
        defer fmt.Printf("%c", v)
    }
}
```

