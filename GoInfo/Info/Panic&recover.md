# Panic and Recover

[TOC]

## 1. Panic ，recover关系

在go中处理不正常状况的最惯用的方法是使用 errors ，错误可以满足大多数的异常处理场景。但是还是有很多状况，使得程序不可以简单地在一个异常后继续执行。在这时就需要一个`panic`来终止程序。当一个panic发生时，程序的运行终止，所有的`deferred`操作都被依次执行，之后程序的控制权返回给调用者。该过程持续进行直到当前goroutines 的所有函数都返回。这时程序根据栈顺序（stack trace）打印panix信息然后终止。

也可以通过`recover`操作从一个panic中恢复该goroutine。`panic`以及`recover`可以被认为与 try-catch-finally 的操作机制相似。可以使用这种方法使得程序更加简洁。

## 2. 使用panic

**注意：需要特别谨慎使用panic，只有当程序无法继续执行时才可以使用panic，否则使用errors进行处理。**一般具有以下两种使用panic的样例：

+ 无法恢复的错误，使得程序无法简单的继续执行。一个例子：当一个web服务器没有成功绑定到一个要求的端口，此时就需要使用一个panic因为绑定失败无法进行下一部动作
+ 编程错误。当有一个method接受一个指针类型的接收器，但是使用一个nil类型变量调用该方法。

### 2.1 panic使用

```go
func panic(v interface{})
	内置函数panic，当一个函数F调用panic函数时，停止当前进程的正常执行，F立即停止执行。被F deferred 的函数按栈顺序正常执行。之后函数F返回到调用者。 
```

```go
package main

import (  
    "fmt"
)

func fullName(firstName *string, lastName *string) {  
    if firstName == nil {
        panic("runtime error: first name cannot be nil")
    }
    if lastName == nil {
        panic("runtime error: last name cannot be nil")
    }
    fmt.Printf("%s %s\n", *firstName, *lastName)
    fmt.Println("returned normally from fullName")
}

func main() {  
    firstName := "Elon"
    fullName(&firstName, nil)
    fmt.Println("returned normally from main")
}
/*panic: runtime error: last name cannot be nil

goroutine 1 [running]:  
main.fullName(0x1040c128, 0x0)  
    /tmp/sandbox135038844/main.go:12 +0x120
main.main()  
    /tmp/sandbox135038844/main.go:20 +0x80*/
```

在使用fullName式产生了panic，一个运行时错误，此时程序的运行终止。根据函数的调用栈，依次打印`main.fullName`, `main.main()` 分别代表`main`goroutine的`fullName, main`方法。

### 2.2 panicking 中使用defer

使用`defer`语句，当一个panic发生时，在程序返回之前一定会调用defer指向的语句，相当于其他语言中的`finally`。

```go
package main

import (  
    "fmt"
)

func fullName(firstName *string, lastName *string) {  
    defer fmt.Println("deferred call in fullName")
    if firstName == nil {
        panic("runtime error: first name cannot be nil")
    }
    if lastName == nil {
        panic("runtime error: last name cannot be nil")
    }
    fmt.Printf("%s %s\n", *firstName, *lastName)
    fmt.Println("returned normally from fullName")
}

func main() {  
    defer fmt.Println("deferred call in main")
    firstName := "Elon"
    fullName(&firstName, nil)
    fmt.Println("returned normally from main")
}
/*deferred call in fullName  
deferred call in main  
panic: runtime error: last name cannot be nil

goroutine 1 [running]:  
main.fullName(0x1042bf90, 0x0)  
    /tmp/sandbox060731990/main.go:13 +0x280
main.main()  
    /tmp/sandbox060731990/main.go:22 +0xc0*/
```

调用fullname中使用了defer语句，所以先出栈，先执行该语句，然后是main中的，最后打印panic信息以及调用栈。

##  3. Recover

```go
func recover() interface{}
	内置函数recover用于管理一个panicked goroutine的行为。在一个deferred函数中执行一个调用使得该goroutine恢复运行。该函数停止panicking序列通过恢复正常的执行，并把错误信息传递给票panic的调用者。注意，recover必须在一个deferred语句中使用，否则无法恢复正常运行。
返回值：当panic的参数为nil，或者recover没有在referred函数中使用，返回nil
否则返回该gouroutine是否正确恢复
```

**注意使用recover时必须在一个deferred函数中使用。**

```go
package main

import (  
    "fmt"
)

func recoverName() {  
    if r := recover(); r!= nil {
        fmt.Println("recovered from ", r)
    }
}

func fullName(firstName *string, lastName *string) {  
    defer recoverName()
    if firstName == nil {
        panic("runtime error: first name cannot be nil")
    }
    if lastName == nil {
        panic("runtime error: last name cannot be nil")
    }
    fmt.Printf("%s %s\n", *firstName, *lastName)
    fmt.Println("returned normally from fullName")
}

func main() {  
    defer fmt.Println("deferred call in main")
    firstName := "Elon"
    fullName(&firstName, nil)
    fmt.Println("returned normally from main")
}

recover from runtime error: the lastName is nil
normally returned from main
defered call from main routine

```

函数`recoverName()`调用了`recover()`函数，返回值传递给`panic`的调用者。现在发生了panic，然后按栈顺序调用deferred函数，所以先执行`recoverName()`函数，该函数使用了`recover()`返回给main.main然后程序继续执行，所以打印最后的语句；然后执行下一个deferred函数。

### 3.1 panic , recover, goroutines

如果使用不同goroutines的recover函数，不可以使得当前goroutine恢复。

```go
package main

import (  
    "fmt"
    "time"
)

func recovery() {  
    if r := recover(); r != nil {
        fmt.Println("recovered:", r)
    }
}

func a() {  
    defer recovery()
    fmt.Println("Inside A")
    go b()
    time.Sleep(1 * time.Second)
}

func b() {  
    fmt.Println("Inside B")
    panic("oh! B panicked")
}

func main() {  
    a()
    fmt.Println("normally returned from main")
}

```

以上代码中，在goroutine b中发生了panic但是使用a中的recover函数无法进行恢复。在a中等待1s使得程序不会在a中中断。如果将`go b()`改为`b()`那么程序就可以从panic中恢复。、

### 4. 运行时错误

panic也可能发生在运行时刻，比如数组越界，变量除0错（注意直接硬码除0错可以被编译器检测，属于语法错误）。关于运行时错误，等价于调用内置函数`panic`，使用一个`runtime.Error`类型的参数。有关`runtime.Error`接口的定义如下：

```GO
type Error interface {
    error
    // RuntimeError is a no-op function but
    // servers to distinguish types that are run 
    // time error from ordinary errors: a type is 
    // run time error if it has a RuntimeError method
    RuntimeError()
}
```

该接口嵌套了`error`接口，所以可以使用`fmt.Print`函数隐式调用`Error（）`方法输出错误信息。`RuntimeError()`方法没有操作，只是用来区分运行时错误以及普通错误。

运行时错误，也可以使用`recover`函数进行恢复。

```go
import "fmt"

func outOfBound() {
  defer recovery()
  a := []int{1, 2, 0}
  fmt.Println(a, a[1]/a[2])
  fmt.Println("normally returned from outOfBound")
}

func recovery() {
  if r := recover(); r != nil {
    fmt.Println("recover successfully")
  }
}

func main() {
  outOfBound()
  fmt.Println("normally returned from main")
}
```

### 4.1 获取栈信息 

如果从一个panic中恢复了goroutine，那么就会失去错误的栈信息（stack trace），可以使用`runtime/debug`包中的`PrintStack（）`函数获取相应的栈信息。

```go
func r() {  
    if r := recover(); r != nil {
        fmt.Println("Recovered", r)
        debug.PrintStack()
    }
}
```

