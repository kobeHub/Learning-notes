# 枚举以及模式匹配

[TOC]

## 1. Enumerations 

枚举类型允许列举所有可能的值来定义一个类型，首先需要定义并且使用一个枚举来展示枚举类型是如何连同数据一起编码信息的。枚举类型中的一个可能的值被称为枚举类型的一个成员。对于一个特定类型的成员的引用，可以使用`::`运算符。一个枚举类型的变量可以指向任何一个成员值。

使用枚举变量，可以在得知所有的可能值的情形下，定义合适的函数签名。

```rust
enum IpAddrKind {
    V4, 
    v6,
}
```

### 1.1 结构体成员包含数据

可以在结构体的每一个成员上定义特定的数据类型，每一个成员都可以包含任意类型，任意数量的数据，可以是基本类型，也可以是结构体类型。可以使用小括号将包含数据括起来。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

如上枚举类型，第一个成员不包含数据；第二个成员包含一个匿名结构体；第三个成员包含一个String，最后一个成员包含三个`i32`类型的数据。注意使用匿名结构体作为枚举类型的成员数据时，不可以使用小括号，需要使用大括号显示结构体的字段。

以上枚举成员也都可以使用结构体进行替代，但是由于每种结构体都是不同的类型，不可以使用统一的变量进行指代。

```rust
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

### 1.2 实现枚举类型上的方法

与结构体类似，可以使用`impl`语句定义枚举类型上的方法。

```rust
impl Message {
    pub fn call(&self) {
        println!("you are calling methods of Message:{:?}", self);
    }
}
```

## 2. `Option`枚举类型

`Option`枚举类型包含了两个成员，`Some， None`，Rust中没有空值的功能。在具有空值的语言中变量总是两种状态之一：空值以及非空值。

Tony Hoare，null 的发明者，在他 2009 年的演讲 “Null References: The Billion Dollar Mistake” 中曾经说到：

> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn't resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

空值的问题在于尝试像一个非空值一样使用空值，会出现形式的错误。但是空值表达的概念是有意义的：空值是由于某种原因目前缺失或者无效的值。使用`Option<T>`枚举表示空值，定义于标准库中。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

该类型被包含在`prelude`中，不需要进行显式的引用，可以直接使用`Some， None`

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

如果使用 `None` 而不是 `Some`，需要告诉 Rust `Option<T>` 是什么类型的，因为编译器只通过 `None` 值无法推断出 `Some` 成员保存的值的类型。当有一个 `Some` 值时，我们就知道存在一个值，而这个值保存在 `Some` 中。当有个 `None` 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。

因为 `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`。例如，这段代码不能编译，因为它尝试将 `Option<i8>` 与 `i8` 相加：

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

如果运行这些代码，将得到类似这样的错误信息：

```rust
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

 

