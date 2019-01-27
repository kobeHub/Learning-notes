# Formatted Print

[TOC]

## 1. `std::fmt`

Rust中的格式化输出由定义在`std::fmt`中的一系列macros实现。主要保函以下宏：

+ `format!`:格式化写入String
+ `print！`：格式化写到终端console (io::stdout)
+ `println!`: 格式化写到终端并且换行
+ `eprint！`：格式化写到标准错误流(io::stderr)
+ `eprintln!`:格式化写到标准错误流并且换行

## 2.格式化输出

### 2.1 使用占位符

占位符`{}`的使用与python中一致，可以使用带有标号的，或者指定名称的。使用宏进行格式化输出时，第一个参数一定是**可迭代的string类型，不可以是一个变量。**之后编译器处理该string并且确定是否有可以传入的占位符参数。

占位符的使用可以使用indice，第一个为0，以此类推。`{}`的意义可以理解为下一个参数，也就是传入宏的下一个参数。使用两种占位符时，各自按照自己的规则进行，第一个`{}`指向1，下一个指向2.

```rust
format!("{1} {} {0} {}", 1, 2); // => "2 1 1 2"
```

```rust
fn format_print() {
    println!("\nFormat print test:");
    let str1 = format!("Just a test with {}", "format!");
    println!("{}", str1);
    println!("{one} is {doing} by {two}",
             one="Tim",
             doing="beating",
             two="Tom");
    println!("{:?}", (12.0, 55));
    println!("{:05}", 31);
    println!("{} of {:b} people know binary, the other do not", 1, 2);
    println!("Padding num with extra 0 {num:>0width$}", num=12, width=5);
}


/*Format print test:
Just a test with format!
Tim is beating by Tom
(12.0, 55)
00031
1 of 10 people know binary, the other do not
Padding num with extra 0 00012
*/
```



### 1.2 参数类型

每一个参数的类型由格式字符串决定。要求每一个参数仅有一个类型的引用。`{index|name:type}`的语法定义了唯一的参数，以及其类型。`{0:x}`表示第一个参数使用16进制输出。对于`{0:x} {0:o}`是不合法的，因为规定了第一个参数既是8进制又是十六进制。

有很多参数不需要一个特定的类型，比如`{：.*}`,第一个参数设置了浮点数的输出中小数点后保留的位数。

```rust
println!("{:>07.*} {1}", 3, 1.3216324);   // 001.322 1.3216324
println!("{:>0wid$.*} {1}", 3, 3.1415, wid=6);   // 003.142 3.1415
print!("{:.*}", 2, 1.245455)   // 1.25
```

注意保留浮点数时按照四舍五入。使用`{m:n.q}` 可以指定哪个位置的浮点数的位数以及小数点后的位数。

### 1.3 精度

对于精度可以使用多种方式,注意使用`$`表示使用的是参数中的精度.

```rust
   // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
println!("Hello {0} is {1:.5}", "x", 0.01);

// Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

// Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

// Hello {next arg ("x")} is {second of next two args (0.01) with precision
//                          specified in first of next two args (5)}
println!("Hello {} is {:.*}",    "x", 5, 0.01);

// Hello {next arg ("x")} is {arg 2 (0.01) with precision
//                          specified in its predecessor (5)}
println!("Hello {} is {2:.*}",   "x", 5, 0.01);

// Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
//                          in arg "prec" (5)}
println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
```
# 2. Traits

当传入一个特定类型的参数时，进行格式化输出。只要该类型实现了以下traits，就可以传入。

- *nothing* ⇒ [`Display`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.Display.html)
- `?` ⇒ [`Debug`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.Debug.html)
- `o` ⇒ [`Octal`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.Octal.html)
- `x` ⇒ [`LowerHex`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.LowerHex.html)
- `X` ⇒ [`UpperHex`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.UpperHex.html)
- `p` ⇒ [`Pointer`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.Pointer.html)
- `b` ⇒ [`Binary`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.Binary.html)
- `e` ⇒ [`LowerExp`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.LowerExp.html)
- `E` ⇒ [`UpperExp`](https://doc.rust-lang.org/1.12.1/std/fmt/trait.UpperExp.html)

这就是是说实现了`fmt::Binary`的任何类型的参数都可以使用`{：b}`进行格式化。标准库为很多原始类型做了实现，如果没有指定格式，那么使用的格式就是`Display`

自定义的类型实现这些traits时使用`self`的引用进行传递。之后函数将输出发送到一个`f.buf`流中。这要求每一个format traits的实现都遵循所要求的格式参数。


**与python比对：**
| *数字*       | *Rust格式*   | *Python格式*   | *输出*             | *描述*                      |
| ------------ | ------------ | -------------- | ------------------ | --------------------------- |
| 3.1415926    | {:.2}        | {:.2f}         | 3.14               | 保留小数点后两位            |
| 3.1415926    | {:+.2}       | {:+.2f}        | +3.14              | 带符号保留小数点后两位      |
| -1           | {:+.2}       | {:+.2f}        | -1(R)/-1.00(P)     | 带符号保留小数点后两位      |
| -1.0         | {:+.2}       | {:+.2f}        | -1.00              | 带符号保留小数点后两位      |
| 2.71828      | {:.0}}       | {:.0f}         | 3                  | 不带小数                    |
| 5            | {:0>2}/{:02} | {:0>2d}/{:02d} | 05                 | 数字补0 (填充左边, 宽度为2) |
| 5            | {:x^10}      | {:x^10d}       | xxxx5xxxxx         | 居中对齐                    |
| 5            | {:x<4}       | {:x<4d}        | 5xxx               | 数字补x (填充右边, 宽度为4) |
| 1000000      | NA.          | {:,}           | 1,000,000          | 以逗号分隔的数字格式        |
| 0.25         | NA.          | {:.2%}         | 25.00%             | 百分比格式                  |
| 1000000000   | NA.          | {:.2e}         | 1.00e+09           | 指数记法                    |
| 1000000000.0 | {:2e}        | {:.2e}         | 1e9(R)/1.00e+09(P) | 指数记法                    |
| 1000000000.0 | {:2E}        | {:.2E}         | 1E9(R)/1.00E+09(P) | 指数记法                    |
| 42           | {:b}         | {:b}           | 101010             | 二进制                      |
| 42           | {:o}         | {:o}           | 52                 | 八进制                      |
| 42           | {:x}         | {:x}           | 2a                 | 十六进制                    |
| 42           | {:X}         | {:X}           | 2A                 | 十六进制                    |
| 42           | {:#b}        | {:#b}          | 0b101010           | 带前缀的二进制              |
| 42           | {:#o}        | {:#o}          | 0o52               | 带前缀的八进制              |
| 42           | {:#x}        | {:#x}          | 0x2a               | 带前缀的十六进制            |
| 42           | {:#X}        | {:#X}          | 0x2A(R)/0X2A(P)    | 带前缀的十六进制            |