# Rust 中的错误处理

[TOC]

## 1. panic 与不可恢复错误

与 golang 相似，可以使用`panic！`调用，产生一个不可恢复的错误。当时用这个宏时，Rust 会打印一个错误信息，展开并清理栈数据。基本的使用场景是检测一些类型的 bug， 并且无法处理。

> **对应panic的栈展开或者终止**
>
> 当出现一个panic时，默认对于栈进行展开（unwinding），将会回溯站并且清理遇到的每一个函数的数据。这种回溯需要进行很多的工作。另一种选择是进行终止，不清理数据直接退出。此时程序拽用的内存有操作系统清理。可以在`Cargo.toml`中添加	模式：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

### 使用`panic！`的 backtrace

可以在运行时，添加`RUST_BACKTRACE=1`的环境变量，查看发生panic时的函数调用栈。

## 2. Result 与可恢复错误

可以使用一个枚举类型来处理潜在的错误，`Result`类型包含两个成员，定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

其中`T` 和`E`是泛型类型参数，可以借助泛型将`Result`类型用于很多场景。实际使用`Result`类型是为了便于进行错误信息的向上传递，也可以进行错误处理。对于常见的可能产生错误的操作，一般都会返回 Result 类型，比如打开文件，文件读写等。

```rust
fn get_file(filename: &str) -> String {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Create file failure: {:?}", e),
            }
            other_error => panic!("There is a problem in opening file:{:?}", other_error),
        },
    };
    let mut contents = String::new();    // Pay atten: read_to_string is a method of &mut File
     match f.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(error) => panic!("Read file error:{:?}", error),
    };
    contents
}
```

以上代码中展示了使用`match`语句对于`Result`类型的处理，可以对于可能的错误类型进行细化，使用`kind()`方法，获取错误的详细类型，从而进行下一步处理。注意从一个文件句柄中读取内容，需要使用一个可变引用`&mut f`。

### 2.1 使用闭包代替 `match`

可以使用闭包代替`match`得到相同的错误处理效果：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}
```

 ### 2.2 `unwrap` and `expect`

`Result<T, E>`类型定义了很多方法处理各种情况，可以使用`unwrap`方法得到其成员值。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

还有另一个类似于 `unwrap` 的方法它还允许我们选择 `panic!` 的错误信息：`expect`。使用 `expect` 而不是 `unwrap` 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。`expect` 的语法看起来像这样：

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

`expect` 与 `unwrap` 的使用方式一样：返回文件句柄或调用 `panic!` 宏。`expect` 用来调用 `panic!` 的错误信息将会作为参数传递给 `expect` ，而不像`unwrap` 那样使用默认的 `panic!` 信息。它看起来像这样：

```rust
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

因为这个错误信息以我们指定的文本开始，`Failed to open hello.txt`，将会更容易找到代码中的错误信息来自何处。如果在多处使用 `unwrap`，则需要花更多的时间来分析到底是哪一个 `unwrap` 造成了 panic，因为所有的 `unwrap` 调用都打印相同的信息。

 ### 2.3 传播错误操作符 `?`

当编写一个其实现调用一些可能会失败的函数时，除了在函数中直接处理该错误；也可以选择让调用者得知这个错误，并决定如何处理，该操作被称为**传播（propagation）**。

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

可以使用一个`？`操作符作为语法糖，如果一个`Result`值为`Ok(T)`,则进行赋值，如果是一个`Err(e)`则函数直接返回。该操作符也可以进行链式使用：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

