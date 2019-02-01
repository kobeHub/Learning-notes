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

有两个方法返回bool值，分别表示由于超时或者临时错误。

### 2.3 直接比较

第三种方法是使用一个`error`类型的变量直接进行比较。*ErrBadPattern* 定义在`filepath`package中：

```go
var ErrBadPattern = errors.New("syntax error in pattern")
```

该错误产生于当filepath包中的Glob函数中传入的pattern不合法时。

```go
package main

import (  
    "fmt"
    "path/filepath"
)

func main() {  
    files, error := filepath.Glob("[")
    if error != nil && error == filepath.ErrBadPattern {
        fmt.Println(error)
        return
    }
    fmt.Println("matched files", files)
}
```

## 3. 自定义错误类型

### 3.1 使用`New（）`函数

创建并且使用自定义错误类型的方法是使用`New()`函数，可以使用`error`包中的`New()`函数，用以创建一个`error`类型的自定义错误类型。返回值是该错误的地址。

```go
// package errors to implements functions to manipulate errors.
package errors

// return an error that formats as the given text.
func New(s string) error {
  return &errorString{s}
}

// errorString is a trivial implementation of error.
type errorString struct {
  s string
}

// implements the error interface, so the variable
// can be assigned to error type.
func (e *errorString) Error() string {
  return e.s
}
```

以上代码定义了一个存储string的错误类型（因为其指针类型实现了`Error（）`方法，所以实现了error接口也就是错误类型，可以使用指针类型或者值类型调用Error,但是error类型变量只可以接收指针类型的`errorString`）。使用`errors.New()`方法可以返回一个`*errorString`类型的指针，使用指针类型可以避免不必要的参数复制。

具体关于该自定义错误类型的使用：

```go
// use the errors.New get a errorString error
func circleArea(radius float64) (float64, error) {
  if radius < 0 {
    return 0, errors.New("The radius must not be be negative")
  }
  return math.Pi * radius * radius, nil
}

// use the fmt.Errorf to get same function
func rectanglArea(leng, wid float64) (float64, error) {
  if leng < 0 {
    return 0, fmt.Errorf("Given length of rectangle %v is not correct\n", leng)
  }
  if wid < 0 {
    return 0, fmt.Errorf("Given width of rectangle %v is not correct\n", wid)
  }
  return leng * wid, nil
}
```

### 3.2 使用`fmt.Errorf()`函数替代

可以使用`fmt.Error()`函数替代上述过程，直接得到更多的关于错误的格式化输出信息。

### 3.3使用结构体类型和域获取错误的更多信息

可以将错误信息作为一个结构体的域，然后使用结构体类型来实现`error`接口，快来作为`errors`类型。使用这种方式提供了更为灵活的错误处理机制。在上述例子中为了获取引起错误的radius，必须调用该错误的描述符。这不是处理错误的正确方法。

可以使用结构体的域对于错误进行访问。产生自定义类型的结构体，一般使用`Error`作为结尾

```go
type areaError struct {
  err string
  radius float64
}

func (e *areaError) Error() string {
  return fmt.Sprintf("radius %0.2f: %s", e.radius, e.err)
}

// use the errors.New get a errorString error
func circleArea(radius float64) (float64, error) {
  if radius < 0 {
    return 0, &areaError{"is nagative", radius}
  }
  return math.Pi * radius * radius, nil
}

func main() {
     radius := -20.
  var holder error
  area, err := circleArea(radius)
  if err != nil {
    holder = err     // 此处得到的err是一个指针类型，使用指针类型是为了避免不必要的参数复制
    fmt.Printf("%v, %T\n", holder, holder)
  } else {
    fmt.Println("Area of the circle:", area)
  }
}

/*radius -20.00: is nagative, *main.areaError*/
```

通过使用自定义错误类型，可以得到期待的所有结果。注意进行接口实现是一般针对指针类型进行实现，从而不需要进行多余的赋值操作。

**注意对于检测一个错误类型是否为nil时，通常使用type asserting进行代替**

### 3.4 使用结构体的方法获取更多错误信息

通过构建自定义错误类型，可以获取更多的错误信息。也可以定义该结构体的方法，返回一个bool值，然后在调用程序中进行进一步的操作。如下例：

```go
type areaError struct {  
    err    string //error description
    length float64 //length which caused the error
    width  float64 //width which caused the error
}
func (e *areaError) Error() string {  
    return e.err
}

func (e *areaError) lengthNegative() bool {  
    return e.length < 0
}

func (e *areaError) widthNegative() bool {  
    return e.width < 0
}

func rectArea(length, width float64) (float64, error) {  
    err := ""
    if length < 0 {
        err += "length is less than zero"
    }
    if width < 0 {
        if err == "" {
            err = "width is less than zero"
        } else {
            err += ", width is less than zero"
        }
    }
    if err != "" {
        return 0, &areaError{err, length, width}
    }
    return length * width, nil
}

func main() {  
    length, width := -5.0, -9.0
    area, err := rectArea(length, width)
    if err != nil {
        if err, ok := err.(*areaError); ok {
            if err.lengthNegative() {
                fmt.Printf("error: length %0.2f is less than zero\n", err.length)

            }
            if err.widthNegative() {
                fmt.Printf("error: width %0.2f is less than zero\n", err.width)

            }
            return
        }
        fmt.Println(err)
        return
    }
    fmt.Println("area of rect", area)
} 
```

