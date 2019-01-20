# Golang 中的指针类型

[TOC]

## 1. 基本的指针声明以及使用

可以使用`*T` 声明一个类型是T 的指针，指针类型用于存储某个变量的地址，使用`*` 操作符可以解引用，获取指针的值，注意golang中的指针不支持指针下移操作,即不支持`ptr++` .

对于一个声明后但是没有指明其值的指针为空指针`nil` 

```go
b := 255
    var a *int = &b
    fmt.Printf("Type of a is %T\n", a)
    fmt.Println("address of b is", a)

a := 25
    var b *int
    if b == nil {
        fmt.Println("b is", b)
        b = &a
        fmt.Println("b after initialization is", b)
    }

```

## 2,使用指针类型作为参数传递

注意golang中一般不使用指针类型作为函数的参数进行传递，而是使用切片类型代替。例如需要更改一个数组的值，可以使用传入一个数组的指针类型，使得内部的数组对于外界是可见的。但是这种方式不符合go的基本思想；使用slice代替

```go
func ch1(ptr *[3]int) {
    ptr[0] = 30
}

func ch2(arr []int) {
    arr[0] = 12
}

func main() {
    a := [3]int{1, 2, 3}
    ch1(&a)
    ch2(a[:])
}
```

使用数组指针时，对于元素的访问需要解引用，也就是使用`*`对于数组元素进行访问。但是可以使用较为简短的语法，直接使用`ptr[i]`对于指针所指向数组的第i个元素进行访问。`（*ptr）[i]-->ptr[i]` 