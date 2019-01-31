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

## 2. `defer`的典型用法

defer语句的主要作用是使得代码会更加简洁明了，通过defer函数或者方法可以对控制流进行改变。典型用例是用在一个`sync.WaitGroup`对象的`Done`方法。

```go
package main

/*Pratical usage cases of the defer statement */
import (
  "fmt"
  "sync"
)

type rect struct {
  length, width float64
}

func (r rect) area(wg *sync.WaitGroup) {
  defer wg.Done()
  if r.width <= 0 {
    fmt.Println("The width of rect must be positive")
    return
  }
  if r.length <= 0 {
    fmt.Println("The length of rect must be positive")
    return
  }
  area := r.length * r.width
  fmt.Println("The area of rect:", area)
}

func main() {
  r1 := rect{12.0, -9.2}
  r2 := rect{11.2, 2}
  r3 := rect{11., -90.}
  rects := []rect{r1, r2, r3}
  var wg sync.WaitGroup
  for _, item := range rects {
    wg.Add(1)
    go item.area(&wg)
  }
  wg.Wait()
  fm
    t.Println("All the routine finished")
}
/*The width of rect must be positive
The width of rect must be positive
The area of rect: 22.4
All the routine finished
*/
```

由于在进行条件判断时，每次在程序返回前都必须执行`wg.Done()`方法，通过使用defer语句不仅使得代码更加简洁，而且当有新的条件需要加入时，不需要担心是否添加了`wg.Done（）`方法。