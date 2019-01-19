# Go struct

[TOC]

## 1. 结构体的组成

在golang中结构体与c中的相似，是一些域的集合，适用于将多个数据域作为同一个抽象概念的场景。结构体的定义以及声明需要使用大括号。使用关键字`type` 定义一个结构体，结构体中的一个每一个属性需要明确定义其名称，多个相同类型的属性可以写在一行并且使用逗号隔开。

也可以不使用关键字`type` 定义结构体，可以使用`var`定义一个匿名结构体。没有新定义一个类型。

### 1.1 声明一个结构体

```go
type People struct {
    firstName string
    lastName string
    age int
}

var People struct {
    firstName, lastName string
    age int   //不需要逗号，因为类型后面不会自动补;
}
```

