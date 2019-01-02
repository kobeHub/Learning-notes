# Golang Map 类型

[TOC]

## 1. 基本用法

golang的map类型与python中的dict类型相似，是一个键值-值的对的集合。该类型是一个引用类型，不是按值传递，所以一个变量可以作为另一个map对象的引用。基本的map类型声明，可以使用以下形式

```go
var testMap map[string]int   // nil map,只是变量声明
test := map[string]int{}     // not nil， 相当于使用了make函数
test := make(map[string]int)
test := map[string]int {
    "First": 1,
    "Second": 2,
}
test["Third"] = 3
```

对于一个map类型的变量，如果只是使用var进行声明便是一个`nil` 空集，如果使用短命名方式，实际上是初始化了一个map类型，相当于调用了make函数，此时不是一个空集。虽然map的长度为0但是不是空

### 添加元素

可以直接使用数组式赋值的方式进行元素添加，也可以在初始化时即将元素添加到map中去

### 遍历元素

可以使用`range`函数进行map遍历，获取key, value

```go
func equal(a, b map[string]int) bool {
  if len(a) == len(b) {
    for key, value := range a {
      got, ok := b[key]
      if ok {
        if got != value {
          return false
        }
      } else {
        return false
      }
    }
  } else {
    return false
  }

  return true
}
```

以上代码用于检验两个map是否相等，不可以直接用==操作符，因为map不是基本类型。需要遍历每一个元素，确定每个元素的键值value都相同后才可判断map相等

### 确定一个元素是否在map中

```
value, ok = map[key]  
// 如果该键值对应的元素在mao中，则返回为真，否则为假
```

### 删除元素

```go
delete(map, key)
```

### 引用类型

```go
package main

import (  
    "fmt"
)

func main() {  
    personSalary := map[string]int{
        "steve": 12000,
        "jamie": 15000,
    }
    personSalary["mike"] = 9000
    fmt.Println("Original person salary", personSalary)
    newPersonSalary := personSalary
    newPersonSalary["mike"] = 18000
    fmt.Println("Person salary changed", personSalary)

}
```



