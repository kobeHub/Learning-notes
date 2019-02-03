unc5Reflection

[TOC]

## 1. 反射的概念

反射是指一个程序在运行过程中，检查自身变量类型的能力。对于静态类型语言来说，所有的变量在编译时都需要明确知道其类型。这对于大多数情况来说都是适用的，但是也有一些特殊情况。如果一个函数使用空接口作为参数，那么可以传入任意类型的变量作为参数。在函数内部需要针对某种特殊的类型进行特定的操作。这是就需要使用到反射机制。

那么什么时候该使用反射机制呢？

```
clear is better than clever. Reflection is never clear.
```

反射是一种强大的机制，但是需要小心使用。因为使用该机制使得代码不够清晰。要尽可能少的使用。

## 2. 基本用法

在go中使用`reflect`包来确定底层真实类型以及值的确定。常用的一些基本函数如下：

+ `reflect.TypeOf(i interface{})`:

  根据i的动态类型，给定其反射类型。如果是空接口，那么返回nil

+ `reflect.ValueOf(i interface{})`:
  根据接口i中的具体类型初始化一个新的value，如果接口为空值，那么返回一个零值。

+ `func (k Kind) String() string`:

  定义了`type Kind uint`,将类型的种类映射到无符号数，然后可以使用`String()`方法得到对应的类型名（基本类型，struct, int）

+ `func (v Value) NumField() int`:

  返回一个结构体v中的域的数量，如果v不是结构体就会报错。该方法是定义在`reflec.Value`结构体上的方法，获取该类型需要使用`reflect.ValueOf()`,获取对应的Value类型后，可以调用该方法得到域的数量。

+ `func (v Vlaue) Field(i int) Value`:

  返回一个结构体的第i个域的值，注意需要使用`ValueOf（）`先获取结构体的值。

+ `Int(), String()`:

  将获得的value转化为int或者string类型

## 3. 典型用例

对于一个构造sql查询语句的函数而言，对于传入的结构体类型，可以根据其具体类型以及域创建相应的查询语句。为了可以传入各种类型的结构体，使用空接口作为参数。

```go
// The order contains id, consumer
type order struct {
  ordId int
  customerId int
}

// employee struct
type employee struct {
  name string
  id int
  address string
  salary int
  country string
}

// create sql statements according to the struct
func createQuery(i interface{}) {
  if reflect.ValueOf(i).Kind() == reflect.Struct {
    v := reflect.ValueOf(i)
    /*
    fmt.Println("Number od fields:", v.NumField())
    for i := 0; i < v.NumField(); i++ {
      fmt.Printf("Field:%d, type:%T, %v, value:%v\n", i,
        v.Field(i), reflect.TypeOf(v), v.Field(i))
    }*/
    t := reflect.TypeOf(i).Name()
    query := fmt.Sprintf("insert into %s values(", t)
    for i := 0; i < v.NumField(); i++ {
      switch v.Field(i).Kind() {
      case reflect.Int:
        if i == 0 {
          query = fmt.Sprintf("%s%d", query, v.Field(i))
        } else {
          query = fmt.Sprintf("%s, %d", query, v.Field(i))
        }
      case reflect.String:
        if i == 0 {
            query = fmt.Sprintf("%s\"%s\"", query, v.Field(i))
          } else {
            query = fmt.Sprintf("%s, \"%s\"", query, v.Field(i))
          }
      default:
        fmt.Println("unsupported type")
      }
    }
    query = fmt.Sprintf("%s)", query)
    fmt.Println(query)
    return
  }
  fmt.Printf("unsupported type")
}

func main() {
    emp1 := employee{
    "Kobe Bryant",
    215667868,
    "Los Angle",
    897868798,
    "America",
  }

  ord := order{
    45345345345,
    676876867876,
  }
  createQuery(emp1)
  createQuery(ord)
}
/*insert into employee values("Kobe Bryant", 215667868, "Los Angle", 897868798, "America")
insert into order values(45345345345, 676876867876)
*/
```

