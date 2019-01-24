# Rust 通用编程概念

[TOC]

## 1. 关键字与标识符

Rust中包含了一系列关键字用于程序的预定义，包含了已经有明确意义的关键字以及没有赋予明确功能供以后使用的关键字。对于所有的变量，函数，结构体等都需要一个名称，这些名称就是标识符，`identifier` 他们可以使任意非空的ASCII字符，但是通常有以下限制.

+ 首字符是字母
+ 其他字符由数字以及字母，_组成
+ 第一个字符是_,但是多于一个字符组成
+ 单独的`_`为占位符，可以用以指示任何一个不使用的变量，与golang相似

### 原始标识符

有时出于某种原因你可能需要将关键字作为名称。比如你需要调用 C 语言库中名为 *match* 的函数，在 C 语言中 *match* 不是关键字。为此你可以使用 “原始标识符”（“raw identifier”）。原始标识符以 `r#` 开头：

```rust
let r#fn = "this variable is named 'fn' even though that's a keyword";

// 调用名为 'match' 的函数
r#match();
```

## 2.变量与可变性

在Rust中变量默认都是不可变的（immutable），有助于充分利用Rust的安全性以及简单并发性的。如果需要使用可变变量时，需要使用`mut`关键字。Rust使用默认不可变的变量，保证了声明一个变量时，如果确定其不可变，那么在代码的生命周期中，该变量的值都不会改变，否则编译器会报错`cannot assign twice to immutable variable x`，所以就不需要追踪某一个变量的值，使得代码更加友好。

声明一个变量时，可以跟上其类型，使用`let var_name: type = ...` 

```rust
fn immutable_test() {
    let num1 = 123;
    let mut str1 = "Just change it!";
    println!("Before change: {} {}", num1, str1);
    str1 = "Now it changed";
    println!("After change: {}", str1)
}


Before change: 123 Just change it!
After change: Now it changed
```

### 常量

使用`const`关键字声明一个常量，虽然变量是不可改变的，但是和常量不同。常量不是默认不改变，而是完全不可以改变；不可以使用`mut`关键字；常量可以在任何作用域中进行声明；常量的赋值只可以使用常量表达式进行赋值，不可以运行时赋值。这与其他语言的常量概念相同。

经常使用常量来定义一些基础值，作用域全局作用域。更改时，只需要更改一处的硬编码。

### 变量隐藏 （Shadowing）

使用`let`关键字重新声明一个已经声明过的变量，那么该变量的类型以及值都发生改变，相当于将之前的变量隐藏。

所以借助隐藏变量的性质，可以“实现”其他语言熟悉的可变变量的用法。但是这种用法与使用`mut`是有很大区别的，每次使用let都是在创建一个新的变量，赋予新的类型以及值。使用`mut`不需要进行新变量的创建.

```rust
let a = 123
let a = a * a
let a = a + 1

let mut b = 123
b = b * b + 1
```

## 3. 数据类型

在Rust中每一个值都有一个数据类型，可以讲数据类型分为两大集合**标量（scalar）**以及**复合类型（compound）**。Rust是一个静态类型的语言，所以所有的变量都必须在编译时就明确知道其类型以及使用方式。当进行类型转换时，如果有多种可能的类型，必须显式指出所转换的类型。

```rust
let guess: u32 = guess.trim().parse()
	.ecpect("Not a int");
```

### Scalar

标量类型代表一个单独的值，Rust中标量主要有四种：整数，浮点数，布尔类型，字符类型。

+ 整形

  按照整数的表示位数，可以进行以下划分

  | length  | signed | unsigned |
  | ------- | ------ | -------- |
  | 8 bits  | i8     | u8       |
  | 16 bits | i16    | u16      |
  | 32 bits | i32    | u32      |
  | 64 bits | i64    | u64      |
  | arch    | isize  | usize    |

  有符号数和无符号数的区别与其他语言相同，有符号数以补码形式存储。有符号数的范围$-(2^{i-1})to2^{n-1}-1$,无符号数范围$0to2^{n}-1$。另外，`isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。`isize`, `usize`主要用作某些集合的索引。

  在Rust中整形的默认类型是`i32`,通常被认为是最快的，即使在64位架构的机器上也是最快的。

  **注意除 byte 以外的所有数字字面值允许使用类型后缀，例如 `57u8`，同时也允许使用 `_` 做为分隔符以方便读数，例如`1_000`。**

+ 浮点型

  Rust中原生的浮点类型有两种`f32, f64`,默认使用64位的浮点类型。因为在现代cpu中32为浮点数的运算速度与64位几乎相同，但是后者精确度更高。

+ bool

  rust是强类型语言，不可以使用其他类型代替bool类型

+ 字符类型

  `char`类型是最原生的字符类型，使用单引号括起来。它与其他语言中的char类型不同，不是映射到ASCII码，每一个char类型代表了一个Unicode标量值。可以表示任何的Unicode字符。

  注意：byte类型使用`b'...'`的形式进行定义，这与golang中的byte类型相同，Rust中的char相当于golang中的rune的作用。

### 复合类型

复合类型（Compound types），将多个值合成为一个类型。包含两个原生的复合类型：元组（tuple），数组(array).

#### 元组类型

元组可以将多个值合并为一个集合的形式，每一个位置可以接受不同类型的值。now iteral the array:
[12 9 8 11 ]

```rust
let tuple = (3.3f32, "Just a test", b'f', 12);
println!("The first ele of tuple: {}", tuple.0);
assert_eq!(tuple.0, 3.3);
```

对于元组中的数据，可以直接使用`.`操作符进行获取，但是标准库中暂不支持遍历操作，可以自定义方法遍历。

#### 数组类型

数组的概念与c中的一致，只存放相同的类型，在内存中的存储空间连续。同时可以使用变长数组`vector`。数组的类型可以表示为`type:; number`,分别表示数组的元素类型、元素数目。这两个值都是不可变的。

**遍历操作：**

```rust
println!("now iteral the array:");
let array = [12.0, 9.0 ,8., 11.];
print!("[");
for ele in array.iter() {
        print!("{} ", ele);
}
println!("]");

// output
now iteral the array:
[12 9 8 11 ]
```



## 4. Addition

### 整形的溢出

对于一个整形变量而言，如果存输的值超过其范围，就会造成溢出。但在debug模式下编译代码时，Rust会检查这些问题并且使得程序`panic`,因错误而退出。

但是在release模式下，Rust不检测溢出。将溢出值进行`two‘s complement wrapping`的操作，`255u8`-->`0u8`, `257u8`-->`1u8`.如果程序要使用这种错误可以使用标准库中的`wrapping`。



## 5. 函数

在Rust中的函数命名规则使用`snake case`的规则，也就是函数名使用小写字母，使用下划线分隔开。Rust的函数签名概念与其他语言一致，参数是函数签名的一部分。函数定义是需要明确形参的类型，然后使用实参传入。函数体由表达式以及语句组成。

### 语句与表达式

Rust与其他语言的一大区别在于它是一门基于表达式的语言**(expression-based)**.语句**（statement）**与表达式之间有着巨大的区别。语句是执行一些操作，但是没有返回值，通常使用`；`隔开，最基本的定义变量，定义函数`fn name() {}` `let a = 10;`都是语句，都不返回任何值。表达式会计算出某些值并且返回`let a= 9;`中9是一个表达式，返回了一个值9。

```rust
fn test() {
    let a = 12;
    let b = {
        let x = 10;
        x * x
    };
}
```

表达式也可以是一个代码块因为最后一个语句没有用分号隔开，所以会返回一个值。所以对于只有一种返回值的函数，可以在最后不使用`return`，直接返回一个表达式。

```rust
fn five() -> i32 {
    5
}
```

## 6. 控制流

### 1. 条件控制

`if`表达式的使用方法与golang一致，对于多条件可以使用`if else`。

```rust
let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
```

注意if语句中只可以使用bool类型进行判断。

**let中使用if**

```rust
  let str3 = if tuple.3 > 8 {
        "It's greater than 8"
    } else {
        "It's less than 8"
    };
    println!("{}", str3);

```

可以使用条件判断进行变量定义，但是注意最后一定要是表达式，不可以加分号。同时，不同的条件分支的表达式的类型必须相同，默认按照第一个分支的类型，不可以出现多种类型。

### 2. 循环

+ `loop` 构建一个无限循环体，可以设置循环终止条件

  ```rust
  let a = 1;
  loop{
      if a > 5 {
          break;
      }
      ...
  }
  fn main() {
      let mut counter = 0;
  
      let result = loop {
          counter += 1;
  
          if counter == 10 {
              break counter * 2;
          }
      };
  
      assert_eq!(result, 20);
  }
  ```

+ `while` 

  rust支持最常用的while操作

  ```rust
  while a > 10 {
      a--
  }
  ```

+ `for`遍历集合

  对于数组等可遍历类型，可以使用for循环进行遍历操作

  ```rust
  fn main() {
      for number in (1..4).rev() {
          println!("{}!", number);
      }
      println!("LIFTOFF!!!");
  }
  ```

  

