# Go中的错误处理

[TOC]

## 1. Error usage

golang中错误的使用与其他语言中的类似。go中的错误类型使用的是内置的`error`类型。和其他内置类型一样，可以使用变量存储错误类型，作为函数或者方法的返回值。

可以使用多种方式进行错误的处理，对于可能的错误如果不加以处理，那么会造成不可预测的错误。可以有很多方法进行错误处理。

### 1.1 获取错误类型与nil比较

在go中`error`是一个接口类型，有着唯一的方法`Error()`,任何实现了该方法的类型都默认实现了该接口。也就有了错误处理机制。

```go
type error interface {
    Error() string
} 
```

该方法提供了一个类型的错误描述符。当打印一个error信息时，println函数调用该类型的错误描述符。

```go
// use the type asserting to get more information
func open_error() {
  f, err := os.Open("tet")
  if err != nil{
      fmt.Printf("Err:%v, type:%T, path: %v\n", err, err)
      return
  }
  fmt.Println("Open succeccfully", f.Name())
}
```

上面代码试图打开一个文件，如果文件不存在会返回一个错误类型。可以使用`err != nil`,进行比较，观察是否产生错误。可以使用type asserting `a.(Type)`,获取有关错误的更多信息。

关于文件打开错误的定义如下：

```go
type PathError struct {
    Op string
    Path string
    Err error
}

func (e *PathError) Error() string {return e.Op + " " + e.Path + e.Err.Error()}
```

## 2. 获取错误更多信息的方法

对于`error`这一接口类型，可以使用打印错误描述符获取其详细信息。对于以上例子的文件路径错误，如果需要获取文件的路径。通过打印描述符是一个糟糕的方法，因为错误描述符可能在新的版本中随时改变，代码也就无法继续生效。

可以通过很多方法获取错误的详细信息。

### 2.1 Assert `a.(Type)`从结构体的域中获取更多信息

```go
package main

import (  
    "fmt"
    "os"
)

func main() {  
    f, err := os.Open("/test.txt")
    if err, ok := err.(*os.PathError); ok {
        fmt.Println("File at path", err.Path, "failed to open")
        return
    }
    fmt.Println(f.Name(), "opened successfully")
}
```

通过类型假设，对于错误类型有了更为精确的比较，返回的`ok`为bool类型。如果为真就可以确定该错误的具体类型，然后使用该类型中的属性的信息。

### 2.2 Assert `a.(Type)` 使用方法获取更多信息

通过比较某个错误类型是否属于一个具体的类型，然后调用该类型的方法获取更多信息。已知`DNSError`的定义如下：

```go
type DNSError struct {  
    ...
}

func (e *DNSError) Error() string {  
    ...
}
func (e *DNSError) Timeout() bool {  
    ... 
}
func (e *DNSError) Temporary() bool {  
    ... 
}
```

有两个方法返回bool值，分别表示由于超时或者临时