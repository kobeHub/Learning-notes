# 结构体的使用

[TOC]

## 1. 定义结构体

使用关键字`struct`定义结构体，定义每一个**字段（field）**名称以及对应的数据类型。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

也可以定义类似元组的结构体，可以使用`（）`进行定义，不需要指明每一个字段的名称，只需要指明每一个字段的类型，以及结构体的类型。**注意，即使具有相同字段的元组结构体，但是类型不同，不可以相互间进行赋值**。使用元组结构体中的字段时，可以使用点操作符，后面跟上字段的顺序。`color.0, color.2`.

```rust

```

### 没有任何字段的结构体

可以定义一个没有任何字段的结构体！它们被称为 **类单元结构体**（*unit-like structs*）因为它们类似于 `()`，即 unit 类型。类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。



## 2. 结构体变量的创建

通过为每一个字段指定一个具体值来创建一个**实例**。对于普通结构体使用大括号以及`key: value`创建，元组结构体使用小括号,以参数形式构建。为了获取某一个实例的属性，可以使用点操作符。需要改变某一个实例的属性，需要将实例定义为可变类型。**注意整个实例可以是可变的，不可以是某些字段可变，某些不可变。**

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

 ### 2.1 变量与字段同名

当变量与字段同名时可以省略字段的名称，直接使用变量构建结构体。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### 2.2 使用其他实例的值构建实例

可以使用结构体更新语法构建新的变量，可以直接使用点操作符，指定某些字段。也可以使用`..ins`指定之后的字段都是用ins的值。

```rust
 let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

## 3. 结构体的所有权

 如果在结构体的字段中使用不拥有所有权的值，比如`&str`，必须指明生命周期。为了使得结构体拥有其所有数据，也就是结构体有效时，所有的属性也有效。

## 4. 结构体的方法

对于结构体的使用，可以定义常规的函数，将结构体作为参数进行使用。也可以在结构体上定义相应的方法，通过`impl`语句块实现相应的功能。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

为一个结构体定义方法的方式与golang中的概念相似。通过使用`impl`语句块，保证了方法的接收器是某种确定的类型。注意对于方法语句，第一个参数需要是`self`，可以根据是否需要传递所有权，是否需要改变变量确定是否使用引用。`&self， &mut self`。上面的`area`方法通过借用一个`Rectangle`变量实现了相应的计算。由于这里的使用不需要获取所有权，只进行变量的读取，所以**借用**即可满足需求。如果需要进行变量值的写入，可以使用可变引用`&mut self`。

只使用`self`作为方法的第一个参数比较少见，一般适用于方法将`self`转化为别的实例时。

使用方法替代函数的好处：

+ 不需要在每一个函数中重复`self`的类型
+ 提高了代码的可组织性，将关于一个类型的方法全部放入一个代码块中，而不是要在所有的函数中寻找相应代码

### 4.1 自动引用与解引用

Rust与golang在方法的处理上就有相似的手法。一个方法定义时的第一个参数类型是引用，如果需要调用时，可以不使用引用类型，而是直接使用值进行调用。编译器自动进行引用操作。如果是一个指针类型调用方法，自动进行解引用操作。

**当使用 `object.something()` 调用方法时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 `object` 与方法签名匹配。**

### 4.2 关联函数（associated function）

可以在`impl`块中定义第一个参数不是`self`的方法，从而实现关联函数。这些函数与结构体相关，但不是结构体的方法，任然是函数。一个典型的实例是`String::from`就是一个关联函数。

同时，对于一个结构体，可以定义多个`impl`块分别用来定义几个相对独立的方法块。

```rust
// methods impl rectangle
impl Rectangle {
    pub fn area(&self) -> u64 {
        self.width * self.length
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // associated function of the struct
    pub fn Square(size: u64) -> Rectangle {
        Rectangle{width: size, length: size}
    }

    pub fn Increase(&mut self, wid: u64, len: u64) {
        self.width += wid;
        self.length += len;
    }
}
```

### 4.3 通过 trait 增加其他功能

关于`println！`宏可以处理格式化输出。对于自定义类型如果使用`{}`进行格式化输出，输出的是`std::fmt::Display`.对于所有的基本类型，都实现了该trait。但是对于结构体类型，由于输出的格式是不明确的。**Rust不会试图猜测用户的意图，所以需要实现相应的trait。**

如果使用`{：？}`的格式需要实现`std::fmt::Debug`trait。可以通过注释在结构体定义前显式添加 `#[derive(debug)]` 实现默认的打印调试信息的功能。使用`{：#？}`可以输出详细的定义。