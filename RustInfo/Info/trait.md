# trait

[TOC]

## 1. trait 作用

类似于很多语言中的接口，Rust 使用 trait 定义一些类型的共享功能。使用 trait 以一种抽象的方式定义共享行为。trait 通常与反省一起使用，通过`trait bound`指定泛型是任何拥有特定行为的类型。

trait 定义了一个类型需要实现的所有行为，只给出函数签名。

## 2. trait 使用

### 2.1 定义一个 trait

使用`trait`关键字定义一个 trait 。一般定义在库项目中，为了其他的项目可以使用，定义为`pub`。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

然后可以为一个具体类型的实现该 trait，**但是注意，必须位于同一个crate的本地作用域下定义的类型才可以实现 trait。** 这个限制与golang一致，主要为了满足相干性（coherence），或者叫孤儿规则（orphan rule），得名于不存在父类型。如果没有这个规则的话，对于同一个类型可以在两个不同的 crate 中实现相同的 trait， 那么调用时就会出现混淆。

```rust
// Article online
pub mod online {
    pub trait Summary {
        fn summary(&self) -> String;
    }

    // News article type
    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // Tweet limits words of 0-280
    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} {}", self.headline, self.author, self.location)
        }
    }

    impl Summary for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

}
```

而且注意需要使用一个 trait 的方法时，需要将其引入当前作用域。 

### 2.2 默认实现

为trait中的某些或者全部方法提供默认的行为是很有用的。当某个特定类型实现 trait 时3可以选择保留或者重载 trait 的方法。这是与其他语言的接口的不同之处。对于使用默认trait默认实现的类型，可以在`impl`块中不实现。

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

默认实现允许调用相同 trait 中的其他方法，即使这些方法没有默认实现。如此，trait 可以实现很多有用的功能，而只需要实现一小部分特定的内容。

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

## 3. Trait bound

使用 trait 作为参数，配合泛型进行使用，可以接受多种类型的参数，实现代码的多态化。可以指定一些实现了某个trait的类型作为参数，使用关键字`impl trait`：

```rust
fn foo(item: impl Summary);
```

这种方式适合于较短的例子，可以写作一个较为完整的语法糖，称为 Trait bound，也就是以 trait 作为类型的边界，规定一个函数或者方法可使用的类型范围。

```rust
fn foo<T: Summary>(item: T);
```

使用 trait bound，可以使得多参数的语法更为明了。同时可以使用`+`连接多个trait用以表示实现了多个trait的类型的泛型。

```rust
fn foo<T: Summary + Display, U: Summary + Clone>(i1: T, i2 U);
```

### 通过 where 简化代码

但是使用trait bound也有自己的缺点，如果泛型参数过多，那么参数列表长度很长，可读性较低。可以使用`where`在方法内部指定泛型参数的trait。

```rust
fn some<T, U>(T: T, u: U) -> i32 {
    where T: Display + Clone,
    	  U: Clone + Debug
}
```



