# Golang Bases

[TOC]

## 1. Why Golang

golang 是一个开源，编译的，静态类型的编程语言．其主要目的是更加简单的开发高可用性，可扩展的web应用．虽然有很多可以完成相同工作的语言，但是与之相比，golang有以下优势:

+ 并发性是该语言固有的部分,更加简易的进行多进程编写,通过`Goroutines` `channels` 实现
+ Golang是编译性的语言,可以产生二进制文件,对于js,python等动态语言不具备该特征
+ 支持静态链接,所有的go代码可以静态连接到一个巨大的二进制库中,可以快速部署到云服务器而不需担心依赖问题 

## ２. Workspace and directory structure

 首先需要建立一个go的工作目录,可以直接在当前终端`export GOPATH=/path/` 作为工作目录,对于golang编译产生的可执行文件,位于当前工作目录下的`bin`文件夹,可以在path中引入该环境变量

`export PATH=${PATH}:${GOPATH}/bin` 

go项目的基本目录结构如下:

```shell
├── bin
│   └── hello
└── src
    └── github.com
        └── nf
            ├── hello
            │   └── echo.go
            └── string
                ├── reverse.go
                └── reverse_test.go
```

+ `src` :

  源代码存放的目录,可以包含多个子文件夹,每个文件夹包含多个包,对于`hello` `string` 是直接可以被引用的包名

+ `bin`:

  可执行文件的位置

对于程序的入口,需要以`package main` 作为包名,进行`go build` `go install` 后生成相应的二进制文件,也可以直接使用`go run xxxx.go` 以类似脚本的形式进行运行,go run 命令只能接受一个命令源码文件以及若干个库源码文件（必须同属于 main 包）作为文件参数，且**不能接受测试源码文件**。它在执行时会检查源码文件的类型。如果参数中有多个或者没有命令源码文件，那么 go run 命令就只会打印错误提示信息并退出，而不会继续执行.

## 3. Syntax

### 1. 代码结构

golang 为静态类型语言,对于每一个变量必须赋予其相应类型,golang相邻的语句间不需要添加`;` 作为间隔,使用{}表示语句块

```go
package main //1

import "fmt" //2

func main() { //3  
    fmt.Println("Hello World") //4
}
```

+ `package main` 每个源代码文件必须以一个包名为开始,标识其所属的包,`main` 包名是程序入口包名

+ `import` 引入所需要的包名,多个包名可以采用()引入,并且可以更名

  ```go
  import (
  	fmt "fmt"
      Str "string"
      "time"
      "math/rand"
  )  // 可以在每个包名后添加;如果不加编译器会自动添加,也可以写为一行,添加分号
  ```

+ `func main` 程序入口

###2. 变量

可以使用多种方式声名变量,可以使用`var`关键字,采用`var variable_name type` 的语法进行变量声明,也可以在声明变量时赋予初始值;在赋予初始值的同时可以省去变量的类型,根据变量值来推测其类型;还可以使用更为优雅的方式,同时声明多个变量;在代码块内部,可以使用短声明的方式声明一个变量`a := 12.0` ,但是不可以用在函数外部

```go
package main

import (
	"fmt"
    "math"
)

var (
	var1 int
    var2 string
    name = "kobe"
    age = 39
)

func main() {
    var test string = "It's just a test"
    var2 = test
    var1 = age
    a, b := 12.6, 22.0
    a, b = b, a
    
    fmt.Println(var1, var2, test, name, age, math.Sqrt(a))
}
```

**注意:使用()进行多个变量声明或者包引入时,每个item间可以用`;`隔开也可以不用,不可以用`,` ** 

### 3. 基本类型

+ bool
+ Numeric Types
  + int8, int16, int32, int64, int
  + uint8, uint16, uint32, uint64, uint
  + float32, float64
  + complex64, complex128
  + byte
  + rune
+ string