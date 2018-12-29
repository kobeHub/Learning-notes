# Goalng 包管理机制

[TOC]

## １．main 函数以及main包

golang 作为典型的静态语言，与其他语言一样需要一个程序入口。每一个可执行的golang程序都有一个`main` 函数入口，该函数一定在`main` package中．在当前工作目录下，在`src` 文件夹下放置源代码，对于不同的模块放在不同的文件夹下，同时，如果含有子模块，可以再建立新的文件夹．

Go 程序的执行（程序启动）顺序如下：

**1.** 按顺序导入所有被 main 包引用的其它包，然后在每个包中执行如下流程：
**2.** 如果该包又导入了其它的包，则从第一步开始递归执行，但是每个包只会被导入一次。
**3.** 然后以相反的顺序在每个包中初始化常量和变量，如果该包含有 init 函数的话，则调用该函数。
**4.** 在完成这一切之后，main 也执行同样的过程，最后调用 main 函数开始执行程序。

## 2. Simple test

在工作目录的`src `文件夹下创建

```shell
├── geometry
│   ├── geometry.go
│   └── rectangle
│       └── rectprops.go

```

`gemotry` 是一个应用程序的主文件夹，在`geometry.go`下使用package main , func main,作为程序入口，同时建立`rectangle`子模块．

进行编译时，首先编译器在根文件夹下寻找一个包含main函数的源文件,然后寻找所有需要引入的文件,进行编译.所有属于同一个包的源文件应该放在他们单独的文件中,最简单的做法是把文件夹名与包名命名一致.

注意进行import时需要从当前工作目录的src开始引入,默认有两个import路径`GOROOT, GOPATH` 一个是系统go根目录,另一个是工作目录`GOROOT=/usr/local/go/src` 

如果需要使用相对路径进行导入,可以使用`.` `./` 开头,否则就是默认的全局寻找方式,在两个go的环境变量域中查找,以`/` 开头则会在绝对目录中查找

除了符号 `_`，包中所有代码对象的标识符必须是唯一的，以避免名称冲突。但是相同的标识符可以在不同的包中使用，因为可以使用包名来区分它们。

```go
import (
  "fmt"
  "github.com/nf/geometry/rectangle"
  "log"
)
```

### 可见性规则

包通过可见性规则,决定是否将自身的代码对象暴露给外部文件.该规则被编译器强制执行.

> 当标识符(变量,常量,类型, 函数名, 结构字段等)以一个大写字母开头,那么这种形式的标识符对象可以被外部包的代码使用
>
> 这被称为exported 相当于java中的`public` ,小写标识符对包外不可见,相当于`private` 

大写字母可以用任何的unicode编码的字符,因此在import一个包之后,只能访问包中导出的对象

对于名称冲突可以使用包的别名方式解决,`import fm "fmt"` 

**注意,如果导入的包没有使用,会产生编译错误**,这是go的宗旨"没有不必要的代码!",可以使用

```go
var _ = unused_package   //不推荐
//但是如果只是想引入一个包,观察其初始化情况,并不调用包中函数,可以使用
import (
	_ "geometry/rectangle"
)
```



## 3. Go程序的一般结构

### Init 函数

每一个包都可以包含一个`init()` 函数, 该函数没有返回值,没有参数,用以执行包的初始化过程．常用操作是通过某些条件判断包是否可以初始化成功，在程序执行前检验正确性．

```go
// Geometry test
package main

import (
  "fmt"
  "github.com/nf/geometry/rectangle"
  "log"
)

var (
  rectLen float64 = 6
  rectWid float64 = 8
)

func init() {
  if rectLen < 0 {
    log.Fatal("length is less than 0")
  }
  if rectWid < 0 {
    log.Fatal("width is less than 0")
  }
  println("Geometry main package initialized")
}
```

**包的初始化过程：**

+ 包的全局变量的首先被初始化
+ 调用`init`函数,一个包中可以有多个初始化函数,可以在单独一个文件或者几个文件中,并且按顺序调用
+ 如果一个包调用其他包,先初始化其他包
+ 被多次引用的包只会被初始化一次

###Demo

下面的程序可以被顺利编译但什么都做不了，不过这很好地展示了一个 Go 程序的首选结构。这种结构并没有被强制要求，编译器也不关心 main 函数在前还是变量的声明在前，但使用统一的结构能够在从上至下阅读 Go 代码时有更好的体验。

所有的结构将在这一章或接下来的章节中进一步地解释说明，但总体思路如下：

- 在完成包的 import 之后，开始对常量、变量和类型的定义或声明。
- 如果存在 init 函数的话，则对该函数进行定义（这是一个特殊的函数，每个含有该函数的包都会首先执行这个函数）。
- 如果当前包是 main 包，则定义 main 函数。
- 然后定义其余的函数，首先是类型的方法，接着是按照 main 函数中先后调用的顺序来定义相关函数，如果有很多函数，则可以按照字母顺序来进行排序。

```go
package main

import (
   "fmt"
)

const c = "C"

var v int = 5

type T struct{}

func init() { // initialization of package
}

func main() {
   var a int
   Func1()
   // ...
   fmt.Println(a)
}

func (t T) Method1() {
   //...
}

func Func1() { // exported function Func1
   //...
}
```

