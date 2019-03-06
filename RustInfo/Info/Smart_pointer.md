# 智能指针

[TOC]

## 1. 智能指针，普通指针，引用

指针是一个包含内存地址的变量的通用概念，这个地址引用或者指向一些其他数据。Rust 中最常见的指针就是引用（reference），属于最基本的指针类型，除了引用数据没有其他的功能，也没有额外的开销，所以使用最多。

**智能指针（Smart Pointer）** 是一类数据结构，表现类似于指针，当时具有额外的元数据以及功能。该概念借鉴于C++， Rust 标准库中的智能指针提供了多于引用的其他功能。例如引用计数（reference counting），可以允许一个变量具有多个所有者，并当没有所有者时负责清理数据。Rust 中的引用不拥有他们所指向的数据，但是智能指针拥有指向数据。例如`String`，`Vec<T>`均属于智能指针。这些类型都属于智能指针因为它们拥有一些数据并允许你修改它们。它们也带有元数据（比如他们的容量）和额外的功能或保证（`String` 的数据总是有效的 UTF-8 编码）。

智能指针使用结构体进行实现，与常规结构体的不同之处在于实现了`Deref`，`Drop` trait。`Deref` trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用又用于智能指针的代码。`Drop` trait 允许我们自定义当智能指针离开作用域时运行的代码。本章会讨论这些 trait 以及为什么对于智能指针来说他们很重要。

## 2. `Box<T>`堆上存储数据

最简单的智能指针是 box，类型为`Box<T>`,允许在堆上存储数据，同时将指向数据的指针留在栈上。除了数据的存储位置不同，box与栈上的数据相比，没有任何性能损失，也没有很多额外的功能。主要用于一下场景：

+ 在编译时未知大小的类型，又需要在确切的上下文中进行使用
+ 大量数据希望在不被 copy 时转移所有权
+ 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

### 2.1 基本methods

`impl<T> Box<T>`

+ `pub fn new(x: T) -> Box<T>`:

  在堆上分配内存，并且将`x`放入其中

+ `pub fn into_raw(b: Box<T>) -> *mut T`

  消费`Box`，并且返回一个原始指针，调用了该方法后，调用者需要对于`Box`之前管理的内存负责。需要显式的对于数据进行内存的释放

+ `pub fn leak<'a>(b: Box<T>) ->&'a mut T`:

  消耗Box并且返回一个可变引用，需要注意的是类型T必须显式注明一个引用的生命周期。如果该类型只有静态引用，或者没有可以使用`‘static`生命周期

  ```rust
  fn main() {
      let x = Box::new(41);
      let static_ref: &'static mut usize = Box::leak(x);
      *static_ref += 1;
      assert_eq!(*static_ref, 42);
  }
  ```

### 2.2 使用 box 定义递归类型

*cons list*（constructor function）是一个来源于Lisp语言的数据结构，两个参数来构造一个新的列表，参数是一个单独的值以及一个列表。cons 函数的概念涉及到更通用的函数式编程术语；“将 *x* 与 *y* 连接” 通常意味着构建一个新的容器而将 *x* 的元素放在新容器的开头，其后则是容器 *y* 的元素。cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 `Nil` 的值并没有下一项。cons list 通过递归调用 `cons` 函数产生。代表递归的终止条件（base case）的规范名称是 `Nil`，它宣布列表的终止。注意这不同于 “null” 或 “nil” 的概念，他们代表无效或缺失的值。

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

如果按照以上方法声明一个递归类型，编译器无法得知我们需要构建的数据的大小，该类型具有无穷大小，因为构造时使用了无法得知大小的List成员，可以使用box解决。

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

`Cons` 成员将会需要一个 `i32` 的大小加上储存 box 指针数据的空间。`Nil` 成员不储存值，所以它比 `Cons` 成员需要更少的空间。现在我们知道了任何 `List` 值最多需要一个 `i32` 加上 box 指针数据的大小。通过使用 box ，打破了这无限递归的连锁，这样编译器就能够计算出储存 `List` 值需要的大小了。

![box](https://doc.rust-lang.org/book/img/trpl15-02.svg)

### 2.3 解引用

为了将智能指针常规指针一样使用，可以使用解引用方式获取指针指向的值。只刷要实现`std::ops::Deref;`trait即可。

```rust
pub mod mybox {
    use std::ops::Deref;
    
    #[derive(Debug)]
    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
}
```

`type Target = T;` 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式。

`deref` 方法体中写入了 `&self.0`，这样 `deref` 返回了我希望通过 `*` 运算符访问的值的引用。没有 `Deref` trait 的话，编译器只会解引用 `&` 引用类型。`deref` 方法向编译器提供了获取任何实现了 `Deref` trait 的类型的值并调用这个类型的 `deref` 方法来获取一个它知道如何解引用的 `&` 引用的能力。

在实际的底层调用如下:

```
*(y.deref())
```

Rust将`*`运算符替换为先调用`deref（）`方法获取值的引用，然后使用普通解引用的方法获取该值。使用这种方式为了不转移`self`的所有权。可以不获取内部值的情况下解引用。

## 3. 解引用强制多态

Deref coercions 时Rust在函数传参上的一种便利操作，可以自动将实现了`Deref`trait的类型转换为引用类型，或者将引用转换为原始类型。可以无需添加过多的`&， *`运算符。

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

因为强制多态，所以该调用是可行的，如果没有该特性，就需要编写`&(*s)[..]`的参数进行调用。当所涉及到的类型定义了 `Deref` trait，Rust 会分析这些类型并使用任意多次 `Deref::deref` 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用解引用强制多态并没有运行时惩罚！

### 3.1 解引用强制多态的可变性

类似于如何使用 `Deref` trait 重载不可变引用的 `*` 运算符，Rust 提供了 `DerefMut`trait 用于重载可变引用的 `*` 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

- 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U`。
- 当 `T: DerefMut<Target=U>` 时从 `&mut T` 到 `&mut U`。
- 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`。

头两个情况除了可变性之外是相同的：第一种情况表明如果有一个 `&T`，而 `T` 实现了返回 `U` 类型的 `Deref`，则可以直接得到 `&U`。第二种情况表明对于可变引用也有着相同的行为。

## 4. 使用`Drop`trait 清理代码

对于智能指针的第二个重要的 trait 事Drop，允许我们在一个值离开作用域时执行的代码，可以任何类型提供`Drop`trait的实现。但一般用于智能指针的实现。在值离开作用域时，按照栈的顺序进行drop（）操作。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

但是不允许显式调用`drop()`method，因为在作用与结束时，会自动调用，为了避免二次释放。为了提早丢弃一个值，可以使用`std::mem::drop`方法。一个例子是当使用智能指针管理锁时；你可能希望强制运行 `drop` 方法来释放锁以便作用域中的其他代码可以获取锁。Rust 并不允许我们主动调用 `Drop` trait 的 `drop` 方法；当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 `std::mem::drop`。`std::mem::drop` 位于 prelude。

```rust
# struct CustomSmartPointer {
#     data: String,
# }
#
# impl Drop for CustomSmartPointer {
#     fn drop(&mut self) {
#         println!("Dropping CustomSmartPointer!");
#     }
# }
#
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

## 5. `Rc<T>`引用计数

大部分的所有权是十分明确的：可以准确得知哪个变量拥有哪个值。然而某些情况单个值可能具有多个所有者。比如图结构中，多条边可能指向同一个结点，而这个结点从概念上讲应该为所有的边所拥有，为了使用多所有权，就需要`Rc<T>`的帮助。即 **Reference Count**。`Rc<T>` 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的那一部分会最后结束使用它的时候

如果希望创建如下的列表：
![two-direct](https://doc.rust-lang.org/book/img/trpl15-03.svg)

继续使用`Box<T>`就会发生所有权的错误，因为对于结点a已经发生了move，已经无法再次使用了。如果使用`Rc<T>`实现的迭代列表，就可以满足该需求。当b创建时不获取a的所有权，而是进行克隆a

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```