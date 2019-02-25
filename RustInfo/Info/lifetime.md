# 生命周期

[TOC]

## 1. 使用情景

Rust 中的每一个引用都有其保持的作用域，称之为生命周期。大部分的引用的生命周期可以进行推断，正如大部分的类型可以推断，但是对于可能存在歧义的情形，必须注明一个具体的类型。所以对于一些引用也需要注明生命周期。以此来确保一个引用一定有效。**某种程度上说，不同于其他语言中的类似工具，可以说这是Rust中最独特的部分。**

### 生命周期避免了垂直引用

垂直引用是指导致程序非预期引用的数据，一般的情形使用一个在一个已经结束的作用中生命的引用。

```rust
{
    let r;
    {
        let x = 5;
        r = &x
    }  // x dropped here while still borrowed 
    println!("{}", r)   // 
}
```

以上代码产生一个编译错误,不允许一个作用与已经结束的变量被借用。

### 借用检查器

Rust 编译器有一个借用检查器（borrow checker），通过比较作用域来保证所有的借用都是正确的。一个有效的引用赋值必须满足**将一个较大的生命周期的变量借用给一个较短的生命周期的引用。**

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}           
```

## 2. 函数中的泛型生命周期

对于一个返回两个字符串切片中的较长者的函数，对于传入的两个`&str`参数，直接的函数实现会报错，因为没有注明生命周期。可以使用注释语法`annoations`

```rust
&i32 
&'a i32
&'a mut i32
```

单个引用的生命周期注释没有太大意义，因为生命周期的主要作用是告诉rust多个引用的生命周期是如何进行联系的。两个具有相同注释的引用的存在时间必须相同。通过注明生命周期，可以实现以上函数：

```rust
pub fn str_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

函数获取的 两个参数都是与生命周期`‘a`存在一样长的字符串slice，函数返回的也是也是与生命周期存在一样长的一个字符串slice。这就是程序告诉Rust必须保证的引用存在的条件，**通过在函数签名中注释生命周期时，并没有改变任何值的生命周期，而是指出任何不遵守这个协议的传入值都会被引用检查器拒绝。**当在函数中使用生命周期注解时，这些注解出现在函数签名中，而不存在于函数体中的任何代码中。这是因为 Rust 能够分析函数中代码而不需要任何协助，不过当函数引用或被函数之外的代码引用时，让 Rust 自身分析出参数或返回值的生命周期几乎是不可能的。这些生命周期在每次函数被调用时都可能不同。这也就是为什么我们需要手动标记生命周期。

但具体传递给函数`str_longest()`时，被`’a`所替代的作用域是`x`的作用域与`y`所重叠的部分，也就是两者中的较小的一个的生命周期。所以返回值的生命周期就必须保证在`x`，`y`中较短的一个生命周期结束前保持有效。

```rust
# fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
#     if x.len() > y.len() {
#         x
#     } else {
#         y
#     }
# }
#
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str())；
    }
    println!("The longest string is {}", result);
}
```

以上代码会报错，因为返回值的生命周期必须是较小的一个，所以在代码块结束时，`string2`已经被dropped，此时`result`的生命周期与`string2`一致，所以也被dropped，所以这个引用就是垂直引用。

### 方法中的生命周期

当为带有生命周期的函数实现方法时，声明和使用生命周期参数的位置依赖于生命周期参数是否同结构体字段或方法参数和返回值相关。结构体字段的生命周期必须总是在 `impl` 关键字之后声明并在结构体名称之后被使用，因为这些生命周期是结构体类型的一部分。

```rust
impl<'a> ImportantExcept<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn except_and_announce(&self, ann: &str) -> &str {
        // According to the second and third rule of lifetime elision
        // the output lifetime can be inferred as &'a self
        println!("Attention please: {}", ann);
        self.except
    }
}

```



## 3. 深入理解生命周期

当在函数中使用生命周期注解时，这些注解出现在函数签名中，而不存在于函数体中的任何代码中。这是因为 Rust 能够分析函数中代码而不需要任何协助，不过当函数引用或被函数之外的代码引用时，让 Rust 自身分析出参数或返回值的生命周期几乎是不可能的。这些生命周期在每次函数被调用时都可能不同。这也就是为什么我们需要手动标记生命周期。

指定生命周期参数的正确方式依赖于函数的具体实现，如果一个函数的返回值一直是第一个参数，那么就不需要指定其他参数的生命周期。以下代码可以被编译：

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {x}
```

**那么当函数返回一个引用，那么就必须与一个参数的生命周期相匹配，如果不与参数的生命周期匹配，那么就肯定和一个内部成员的生命周期相匹配，这样的引用肯定是无效的。**

## 4. 结构体使用生命周期

在一个存放引用的结构体中，必须使用生命周期注释，指定一个引用的可用范围。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

这里的 `main` 函数创建了一个 `ImportantExcerpt` 的实例，它存放了变量 `novel` 所拥有的 `String` 的第一个句子的引用。`novel` 的数据在 `ImportantExcerpt` 实例创建之前就存在。另外，直到 `ImportantExcerpt` 离开作用域之后 `novel` 都不会离开作用域，所以 `ImportantExcerpt` 实例中的引用是有效的.

## 5. 生命周期的特例

### 5.1 省略

现在可以确定每一个引用都有一个生命周期，而且需要为了使用了引用的函数体的参数指定一个生命周期。但是可以根据一些规则省略生命周期的使用。例如以下代码，可以不注明参数的生命周期：

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

出现这种情形是由于一些历史原因，在早期的版本中，该代码的确是无法编译的，因为规定了每一个引用都必须有一个明确的生命周期，那时的函数签名会写作：

```rust
fn first_Word<'a>(s: &'a str) -> &'a str
```

但是编写了很多代码后，发现一些代码的生命周期是可以预测的，不一定需要重复的进行编写，所以加入了**生命周期省略规则（lifetime elision rules）**。讲生命周期分为输入生命周期（包含函数或者方法的签名），以及输出生命周期（函数的返回值）。编译器通过检查以下三条规则，如果可以确定所有引用的生命周期，那么就可以通过编译，否则就编译错误。

+ 每一个引用的参数，都有自己的一个生命周期
+ 如果只有一个输入的生命周期的参数，那么他被赋予所有输出的生命周期。也就是说：`fn foo<'a'>(x: &'a str) -> &'a str`
+ 如果是方法的生命周期参数，具有一个`&self, &mut self`,那么`self`的生命周期就被赋予所有的输出生命周期

那么再来分析以下函数签名：

`fn longest(x: &str, y: &str) -> &str` 两个参数具有两个生命周期，第二条不适用，第三条没有`self`不适用，那么就无法确定输出生命周期，编译失败.

`fn first_word(x: &str) -> &str`,符合第二条规则，可以通过编译 

 ### 5.2 静态生命周期

有一个特殊的生命周期需要考虑，`‘static`其生命周期存在于整个程序期间，是一直可用的。常见的字符串字面值就是一个静态生命周期的引用，被直接存储在二进制文件中。在错误信息的帮助文本中见过使用 `'static` 生命周期的建议，不过将引用指定为 `'static` 之前，思考一下这个引用是否真的在整个程序的生命周期里都有效。你可能会考虑希望它一直有效，如果可能的话。大部分情况，代码中的问题是尝试创建一个悬垂引用或者可用的生命周期不匹配，请解决这些问题而不是指定一个 `'static` 的生命周期。

结合泛型，trait bounds，以及生命周期，由于泛型和生命周期都需要写在`<>`中，所以可以按照以下语法：

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

