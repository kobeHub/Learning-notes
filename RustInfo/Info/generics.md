# 泛型 generics

[TOC]

## 1. 泛型的作用

每一个语言都有一个高效处理重复概念的工具，在 Rust 中使用的是Generics。泛型是具体类型或者其他属性的抽象代替，可以定义泛型，规定一类具有相似行为的类型。为了编写一份可以用于多种具体值的代码，可以使用泛型作为参数或者返回值。一些枚举类型上，`Option<T>`, `Result<T, E>`,`Vec<T>`,`HashMap<K, V>` 都使用了泛型。

可以使用泛型显著减少代码的重复。一般而言，可以使用一个函数处理代码的重复。比如求一个数组最大值的函数。但是有一个问题是，对于不同类型而言，需要编写不同的函数，这显然也是一种重复。此时，泛型就派上用场了。

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

这两个函数有着相同的代码，所以让我们在一个单独的函数中引入泛型参数来消除重复。

为了参数化函数签名中的类型，需要给每一个类型参数起一个名字，任何标识符都可以作为类型参数名。一般类型使用`T`表示，错误使用`E`表示。当一个函数使用一个泛型参数时，需要在函数名后定义该泛型。

```rust
fn largest<T>(list: &[T]) -> T;
```

**但是注意，泛型并不是说所有类型都可以使用。一个泛型指定的范围越大，那么可以使用的公共方法越少。对于泛型的约束通常使用 `trait` 来进行。 ** `trait`以类似接口的形式，定义了一些行为规范，每一个实现了某个trait的类型都必须实现所有方法。

```rust
use std::cmp::PartialOrd;
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // The type which impls PartialOrd and Copy trait can use
    // the function
    let mut largest = list[0];    // Copy trait
    for &item in list.iter() {
        if item > largest {      // PartialOrd trait
            largest = item;
        }
    }

    largest
}
```

上述代码中的赋值操作，需要实现了`Copy` trait 的类型才可以进行访问， 因为list 是一个引用类型，不可以进行move。**否则默认实现 move 操作。**   比较操作需要实现了`std::cmp::PartialOrd`trait的类型才可以调用。

## 2. 结构体中定义的泛型

可以使用`<>`语法定义具有泛型参数类型字段的结构体，如果所需要的泛型字段类型相同，可以使用同一个泛型，如果不一样必须使用更多的泛型，**不同的泛型可以使用同一个类型。**

```rust
#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}


struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}

error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
floating-point variable
  |
  = note: expected type `{integer}`
             found type `{float}`
```

## 3. 枚举中定义泛型

枚举类型的字段可以定义泛型，用以包含不同类型的数据。一个典型用例就是`Option<T>`.`Result<T, E>`.

## 4. 方法中定义泛型

可以为定义中包含泛型的结构体或者枚举类型的定义方法，但是需要在`impl`关键字后添加相应的泛型参数，在类型后面也需要添加泛型参数。

```rust
// Methods for Point
impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &U {
        &self.y
    }
}
```

除了可以为所有泛型定义相同的方法，也可以为泛型赋予特定类型，然后实现相应的方法。此时不需要`impl`后的`<>`

```rust
// Methods for the specific type
impl Point<f64, f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## 5. 泛型代码的性能

Rust实现的泛型，使得使用泛型的代码与使用具体类型的代码具有相同的性能。通过在编译时进行泛型代码的**单态化（monomorphization）**来保证效率。单态化通过填充编译时所使用的具体类型，从而使得每一个泛型在编译时都有唯一的具体类型的与之绑定。这样就实现了编写通用代码，同时得到特定代码性能的目的。

以`Option<T>`为例，进行单态化时，一个对应`i32`,那么单态化时，可以表示为`Option_i32`从而实现了类型的具体化。

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```