# Slice 类型

[TOC]

## 1. 基本使用

除了基本类型，还有一个没有所有权的类型 slice。切片是集合中一段连续的元素序列，概念与go相似，基于原有的数组进行构建。作为连续序列的一个动态的视图。

### 1.1 String slice

可以使用下标来定义一个字符串的引用，`let t = &s[0..5]`,类似于python中的`[0:5]`语法。`start...end`不包含end。如果需要包含，可以使用`let t = &s[1..=6]`。

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

string slice 类型实际上就是字符串字面值类型`str`，对于一个定义的一个字符串字面值

```rust
let s = "test";
```

字符串字面值存储在二进制文件中，s类型是`&str`，是一个指向二进制程序的特定位置的slice，**这也解释了字符串字面值为什么不可变**。

### 1.2 函数参数使用 &str

对于函数签名可以使用`string`类型的都可以使用字符串字面值进行代替。可以对`String &str`使用相同的函数。如果有一个slice可以直接传入，如果是Strings可以传入对应的slice。

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
```



