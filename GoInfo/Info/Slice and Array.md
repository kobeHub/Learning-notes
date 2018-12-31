# 数组以及切片

[TOC]

## 1. Arrays

golang数组与基本静态语言的数组一致，由于golang是强类型的语言，所以数组元素必须类型相同。数组的基本声明方式如下：

###1.1 数组声明

+ 定长数组

  使用`var arr [n]T` 进行声明长度为n, 类型为T的数组对象，采用这种方式进行声明，每个元素根据数组类型赋予相应的初始值，例如int初值为0

  也可以在声明时向数组赋予初始值，`a := [3]int{1, 3, 6}` ,可以不全赋值，其余位置会被默认值填充

  ```go
  // 初始值测试
  package main
  
  import (
  	"fmt"
  	"reflect"
  )
  
  func main() {
  	var a [3]int //int array with length 3
  	
  	var (
  	    b [3]float32
  	    c [3]float64
  	    d [3]string
  	    e [3]bool
  	    f [3]byte
  	    g [3]rune
  	    h [3]complex64
  	    i [3]int8
  	)
  	fmt.Println(reflect.ValueOf(a[0]).Kind(), a,
  	reflect.ValueOf(b[0]).Kind(), b, 
  	reflect.ValueOf(c[0]).Kind(), c, 
  	reflect.ValueOf(d[0]).Kind(), d, 
  	reflect.ValueOf(e[0]).Kind(), e, 
  	reflect.ValueOf(f[0]).Kind(), f, 
  	reflect.ValueOf(g[0]).Kind(), g, 
  	reflect.ValueOf(h[0]).Kind(), h,
  	reflect.ValueOf(i[0]).Kind(), i)
  }
  int [0 0 0] float32 [0 0 0] float64 [0 0 0] string [  ] bool [false false false] uint8 [0 0 0] int32 [0 0 0] complex64 [(0+0i) (0+0i) (0+0i)] int8 [0 0 0]
  ```

  

+ 自定长度数组

  可以使用` [...]T{.., .., } ` 进行数组的声明，编译器会自动根据给定元素给数组确定长度

  但是后面必须跟上数组元素的声明，不可以不指定元素

  ```go
  func main() {
  	a := [...]int{1, 2, 1e3, 2e4} // ... makes the compiler determine the length
  	fmt.Println(a)
  }
  ```

  **注意数组长度也是数组类型的一部分**，对于强类型语言来说，不同长度的数组不属于同一类型，所以将不同长度的数组进行赋值是不被允许的

###1.2. 数组以值传递

golang中的数组是`value type`而不是`reference type` ,也就是说，进行数组间的相互赋值时，使用的不是同一个内存空间，开辟了另一个内存空间用以存放新的数组，所以改变新的数组中元素并不会改变原数组的值，也就是说，使用的是深复制。

所以数组作为参数进行传递时，也是按值传递，不会改变原数组的元素。

```go
func changeLocal(num [5]int) {  
    num[0] = 55
    fmt.Println("inside function ", num)

}
func main() {  
    num := [...]int{5, 6, 7, 8, 8}
    fmt.Println("before passing to function ", num)
    changeLocal(num) //num is passed by value
    fmt.Println("after passing to function ", num)
}
```

关于数组的长度可以使用`len()`函数求取

## 2. 可迭代对象的遍历

对于数组等可迭代对象，可以使用基本的for循环进行，也可以使用`range` 函数，进行迭代，每次迭代包含`index， value`两个值进行使用,如果需要忽略index可以使用空白操作符，忽略value可以只进行index的赋值

```go
for i, v := range list {
    fmt.Println(i, v)
}
```

## 3. 切片

数组虽然已经足够灵活，但是必须是确定长度，不支持变长操作。切片类型是基于数组的一个顶层`wrapper` ,切片本身并不存放任何数据，他们只是某些已经存在的数组的引用

###	3.1 切片声明

```go
a := [5]int{1, 2, 67, 76, 88}
var b []int = a[1:4]
// 1, 2, 67

a := [5]int{76, 77, 78, 79, 80}
var b  = a[1:4] //creates a slice from a[1] to a[3]
fmt.Println(b[1:4], b)  // 注意此处slice的len为3，cap为4，不可以用[]访问，但是可以使用[:]格式进行访问，
```

切片的类型用不需要长度的`[]int`表示，对于数组的`[:]` 操作，与python用法一致，但是不允许负值操作，而且不允许倒序操作，而且最多只接受两个参数，同样，包含下界不包含上界。

**使用`make()` 函数声明切片**

```go
func make([]T, len, cap int) []T
```

用于生成一个切片，传入的是类型，容量以及长度，值采用默认值填充。

### 3.2 切片机制

切片不拥有任何数据，只是作为底层数组的引用，所以基于切片的数据修改，会直接反映在底层数组对象上。所以公用同一个底层数组的切片，如果对其进行更改，会映射到每一个切片上去。

```go
    numa := [3]int{78, 79 ,80}
    nums1 := numa[:] //creates a slice which contains all elements of the array
    nums2 := numa[:]
    fmt.Println("array before change 1",numa)
    nums1[0] = 100
    fmt.Println("array after modification to slice nums1", numa)
    nums2[1] = 101
    fmt.Println("array after modification to slice nums2", numa)
```

**长度与容量：**

切片的长度使用`len（）`函数，表示的是切片可表示的元素的数量；切片的容量是从切片开始位置到数组最后一个元素的长度。

一个切片可以被重新切片，并且增长到其容量，如果超出范围会抛出运行时错误。

```go
fruitarray := [...]string{"apple", "orange", "grape", "mango", "water melon", "pine apple", "chikoo"}
fruitslice := fruitarray[1:3]
fmt.Printf("length of slice %d capacity %d\n", len(fruitslice), cap(fruitslice)) //length of is 2 and capacity is 6
fruitslice = fruitslice[:cap(fruitslice)] //re-slicing furitslice till its capacity
fmt.Println("After re-slicing length is",len(fruitslice), "and capacity is",cap(fruitslice))        // 6, 6
```

**结构特征**

slices可以被表示为一个结构体

```go
type slice struct {
    Length int
    Capacity int
    ZerothElement *byte    // 头指针
}
```

一个切片包含了长度，容量，以及指向数组第一个元素的指针，但一个切片传作为参数传递时，虽然是按值传递，但是指针变量仍然会指向同一个底层数组。所以使用切片进行参数传递时，内部的改变会影响到函数外部.



### 3.3 基本操作

+ `Append()`

  ```go
  func append([]T, x ...T) []T
  // 用于动态扩充一个切片，参数x ...T表示接受多个类型为T的值作为参数，可变长参数
  // 当一个切片进行扩充时，如果长度与容量相等，表示空间已经用完
  // 此时建立一个底层新的数组，容量为原来2倍并且把原数组的数据copy到新的数组中去，形成新的底层数组
  // 并且把新数组的引用赋给切片
  ```

  ```go
  cars := []string{"Ferrari", "Honda", "Ford"}
  fmt.Println("cars:", cars, "has old length", len(cars), "and capacity",cap(cars)) //capacity of cars is 3
  cars = append(cars, "Toyota")
  fmt.Println("cars:", cars, "has new length", len(cars), "and capacity",cap(cars)) //capacity of cars is doubled to 6
  ```

  append不仅可以追加单个或者多个元素，而且可以追加另外一个slice, 使用`...`操作符即可

  ```go
  fruits := []string{"oranges", "apples"}
  veg := []string{"potatoes"}
  food := append(veg, fruits...)
  ```

+ copy

  ```go
  func copy(dst, src []T) int
  ```

  将切片的容量扩充2倍，可以使用

  ```go
  t := make([]byte, len(s), (cap(s)+1)*2)   // cap可能为0
  copy(t, s)
  s = t
  // s = append(s, x ...T)
  ```

  

### 3.4 空值nil与多维slice

一个slice 的length , capacity均为0，则该切片为空`nil` ，对于一个控切片，可以使用`append()` 进行扩充。   

使用多维slice只需要在声明是将其指定为多维即可

```go
pls := [][][]string{
    {
      {"My", "programming"},
      {"Language"},
    },
    {
      {"c", "c++"},
      {"Python", "JavaScript"},
      {"Go", "Rust"},
    },
  }
```

注意使用多维数组时，只允许最高纬度使用`...`运算符，表示可以不确定其维度，但是其他维度必须标识，如果不标识则内部存放的是slice

```go
var te [...][][]string
var te [...][2][3]int
```

##4. 切片的优化

正如前面所说，切片操作并不会复制底层的数组。整个数组将被保存在内存中，直到它不再被引用。 有时候可能会因为一个小的内存引用导致保存所有的数据。

例如， `FindDigits` 函数加载整个文件到内存，然后搜索第一个连续的数字，最后结果以切片方式返回。

```go
var digitRegexp = regexp.MustCompile("[0-9]+")

func FindDigits(filename string) []byte {
    b, _ := ioutil.ReadFile(filename)
    return digitRegexp.Find(b)
}
```

因为切片引用了原始的数组， 导致 GC 不能释放数组的空间；只用到少数几个字节却导致整个文件的内容都一直保存在内存里。

改进方法是将该切片复制到一个新的切片中，释放原内存。

```go
func CopyDigits(filename string) []byte {
    b, _ := ioutil.ReadFile(filename)
    b = digitRegexp.Find(b)
    c := make([]byte, len(b))
    copy(c, b)
    return c
}
// c := make([]byte, 0)
// c = append(c, b...)
```

