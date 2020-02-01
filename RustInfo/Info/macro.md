# Macro

宏指的是Rust中一系列功能： **声明宏（Declatative）**, 使用`macro_rules!`匹配特定的Rust代码，并且进行宏展开；三种**过程宏(Procedural)**:

+ 自定义`#[derive]`宏在结构体以及枚举上通过`derive`属性添加的代码
+ 类属性（Attribute）宏定义可用于任一项的自定义属性
+ 类函数宏类似于函数不过作用于作为参数传递的Token

## 1. 宏与函数的区别

宏是一种为编写代码而写代码的方式，即元编程（metaprogramming）。宏可以扮演函数的角色，但是宏有一些函数所没有的附加能力。

一个函数必须声明函数的参数数量以及类型，相比之下，宏只接受一个可变参数。并在编译器翻译代码前展开，红可以在一个特定类型上实现trait，由于函数实在运行时被调用的，同时trait在运行时实现，所以函数无法像宏一样。实现一个宏而不是函数的消极面是宏定义要比函数定义更复杂，因为你正在编写生成 Rust 代码的 Rust 代码。由于这样的间接性，宏定义通常要比函数定义更难阅读、理解以及维护。

宏和函数的最后一个重要的区别是：在调用宏 **之前** 必须定义并将其引入作用域，而函数则可以在任何地方定义和调用。

## 2. 声明宏

Rust最常用的宏就是声明宏，有时也被称为（Macro by example）, `macro_rules!`宏，“macro”。声明宏允许我们编写一些类似于`match`分支的代码，其接受一个表达式，与表达式的结果进行模式匹配，然后根据匹配执行相应的代码。宏匹配的是Rust的源代码字面值。模式用于和传递给宏的源代码进行比较，同时每个模式的相关代码则用于替换传递给宏的代码。所有这一切都发生于编译时。

以`vec！`宏的简化实现为例：

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), *) => {
        {
            let mut tmp_vec = Vec::new();
            $(
            	tmp_vec.push($x);
            )*
            tmp_vec
        }
    };
}
```

使用`#[macro_export]`注释说明宏是可用的，可以被引入作用域。宏定义中，有一个单边模式`( $( $x:expr ),* )`用于匹配特定模式的源代码，`$`符号内的标识通过替代代码捕获了符合括号内模式的值，`$x:expr` 匹配任意表达式，或者给定名字的表达式，在这个括号后面的`,*`表示匹配该模式0次或多次。

`macro_rules!` 中有一些奇怪的地方。在将来，会有第二种采用 `macro` 关键字的声明宏，其工作方式类似但修复了这些极端情况。在此之后，`macro_rules!` 实际上就过时（deprecated）了。在此基础之上，同时鉴于大多数 Rust 程序员 **使用** 宏而非 **编写** 宏的事实，此处不再深入探讨 `macro_rules!`。请查阅在线文档或其他资源，如 [“The Little Book of Rust Macros”](https://danielkeep.github.io/tlborm/book/index.html) 来更多地了解如何写宏。

## 3. 用于从属性生成代码的过程宏

过程宏(Procedural Macro)更类似于函数。过程宏接受Rust代码作为输入，在这些代码上进行操作，产生另一些代码作为输出，而不是像声明宏一样进行代码替换。

有三种类型的过程宏（自定义 derive，类属性和类函数），不过它们的工作方式都类似。

当创建过程宏时，其定义必须位于一种特殊类型的属于它们自己的 crate 中。这么做出于复杂的技术原因，将来我们希望能够消除这些限制。使用这些宏需采用类似示例 19-29 所示的代码形式，其中 `some_attribute` 是一个使用特定宏的占位符。

过程宏包含一个函数，这也是其得名的原因：“过程” 是 “函数” 的同义词。那么为何不叫 “函数宏” 呢？好吧，有一个过程宏是 “类函数” 的，叫成函数会产生混乱。无论如何，定义过程宏的函数接受一个 `TokenStream` 作为输入并产生一个 `TokenStream` 作为输出。这也就是宏的核心：宏所处理的源代码组成了输入 `TokenStream`，同时宏生成的代码是输出 `TokenStream`。最后，函数上有一个属性；这个属性表明过程宏的类型。在同一 crate 中可以有多种的过程宏。

### 3.1 自定义派生宏

让我们创建一个 `hello_macro` crate，其包含名为 `HelloMacro` 的 trait 和关联函数 `hello_macro`。不同于让 crate 的用户为其每一个类型实现 `HelloMacro` trait，我们将会提供一个过程式宏以便用户可以使用 `#[derive(HelloMacro)]` 注解他们的类型来得到 `hello_macro` 函数的默认实现。该默认实现会打印 `Hello, Macro! My name is TypeName!`，其中 `TypeName` 为定义了 trait 的类型名。换言之，我们会创建一个 crate，使程序员能够写类似示例 19-30 中的代码。

文件名: src/main.rs

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

示例 19-30: crate 用户所写的能够使用过程式宏的代码

运行该代码将会打印 `Hello, Macro! My name is Pancakes!` 第一步是像下面这样新建一个库 crate：

```text
$ cargo new hello_macro --lib
```

接下来，会定义 `HelloMacro` trait 以及其关联函数：

文件名: src/lib.rs

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

现在有了一个包含函数的 trait 。此时，crate 用户可以实现该 trait 以达到其期望的功能，像这样：

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

然而，他们需要为每一个他们想使用 `hello_macro` 的类型编写实现的代码块。我们希望为其节约这些工作。

另外，我们也无法为 `hello_macro` 函数提供一个能够打印实现了该 trait 的类型的名字的默认实现：Rust 没有反射的能力，因此其无法在运行时获取类型名。我们需要一个在运行时生成代码的宏。

下一步是定义过程式宏。在编写本部分时，过程式宏必须在其自己的 crate 内。该限制最终可能被取消。构造 crate 和其中宏的惯例如下：对于一个 `foo` 的包来说，一个自定义的派生过程宏的包被称为 `foo_derive` 。在 `hello_macro` 项目中新建名为 `hello_macro_derive` 的包。

```text
$ cargo new hello_macro_derive --lib
```

由于两个 crate 紧密相关，因此在 `hello_macro` 包的目录下创建过程式宏的 crate。如果改变在 `hello_macro` 中定义的 trait ，同时也必须改变在 `hello_macro_derive` 中实现的过程式宏。这两个包需要分别发布，编程人员如果使用这些包，则需要同时添加这两个依赖并将其引入作用域。我们也可以只用 `hello_macro` 包而将 `hello_macro_derive` 作为一个依赖，并重新导出过程式宏的代码。但我们组织项目的方式使编程人员使用 `hello_macro` 成为可能，即使他们无需 `derive` 的功能。

需要将 `hello_macro_derive` 声明为一个过程宏的 crate。同时也需要 `syn` 和 `quote` crate 中的功能，正如注释中所说，需要将其加到依赖中。为 `hello_macro_derive` 将下面的代码加入到 *Cargo.toml* 文件中。

文件名: hello_macro_derive/Cargo.toml

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.14.4"
quote = "0.6.3"
```

为定义一个过程式宏，请将示例 19-31 中的代码放在 `hello_macro_derive` crate 的 *src/lib.rs* 文件里面。注意这段代码在我们添加 `impl_hello_macro` 函数的定义之前是无法编译的。

文件名: hello_macro_derive/src/lib.rs

> 在 Rust 1.31.0 时，`extern crate` 仍是必须的，请查看
> https://github.com/rust-lang/rust/issues/54418
> https://github.com/rust-lang/rust/pull/54658
> https://github.com/rust-lang/rust/issues/55599

```rust
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}
```

示例 19-31: 大多数过程式宏处理 Rust 代码时所需的代码

注意 `hello_macro_derive` 函数中代码分割的方式，它负责解析 `TokenStream`，而 `impl_hello_macro` 函数则负责转换语法树：这让编写一个过程式宏更加方便。外部函数中的代码（在这里是 `hello_macro_derive`）几乎在所有你能看到或创建的过程宏 crate 中都一样。内部函数（在这里是 `impl_hello_macro`）的函数体中所指定的代码则依过程宏的目的而各有不同。

现在，我们已经引入了三个新的 crate：`proc_macro` 、 [`syn`](https://crates.io/crates/syn) 和 [`quote`](https://crates.io/crates/quote) 。Rust 自带 `proc_macro` crate，因此无需将其加到 *Cargo.toml* 文件的依赖中。`proc_macro` crate 是编译器用来读取和操作我们 Rust 代码的 API。

`syn` crate 将字符串中的 Rust 代码解析成为一个可以操作的数据结构。`quote` 则将 `syn` 解析的数据结构反过来传入到 Rust 代码中。这些 crate 让解析任何我们所要处理的 Rust 代码变得更简单：为 Rust 编写整个的解析器并不是一件简单的工作。

当用户在一个类型上指定 `#[derive(HelloMacro)]` 时，`hello_macro_derive` 函数将会被调用。原因在于我们已经使用 `proc_macro_derive` 及其指定名称对 `hello_macro_derive` 函数进行了注解：`HelloMacro` ，其匹配到 trait 名，这是大多数过程宏遵循的习惯。

该函数首先将来自 `TokenStream` 的 `input` 转换为一个我们可以解释和操作的数据结构。这正是 `syn` 派上用场的地方。`syn` 中的 `parse_derive_input` 函数获取一个 `TokenStream` 并返回一个表示解析出 Rust 代码的 `DeriveInput` 结构体。示例 19-32 展示了从字符串 `struct Pancakes;` 中解析出来的 `DeriveInput` 结构体的相关部分：

```rust
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

示例 19-32: 解析示例 19-30 中带有宏属性的代码时得到的 `DeriveInput` 实例

该结构体的字段展示了我们解析的 Rust 代码是一个类单元结构体，其 `ident`（ identifier，表示名字）为 `Pancakes`。该结构体里面有更多字段描述了所有类型的 Rust 代码，查阅 [`syn` 中 `DeriveInput` 的文档](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html) 以获取更多信息。

此时，尚未定义 `impl_hello_macro` 函数，其用于构建所要包含在内的 Rust 新代码。但在此之前，注意其输出也是 `TokenStream`。所返回的 `TokenStream` 会被加到我们的 crate 用户所写的代码中，因此，当用户编译他们的 crate 时，他们会获取到我们所提供的额外功能。

你可能也注意到了，当调用 `parse_derive_input` 或 `parse` 失败时。在错误时 panic 对过程宏来说是必须的，因为 `proc_macro_derive` 函数必须返回 `TokenStream` 而不是 `Result`，以此来符合过程宏的 API。这里选择用 `unwrap` 来简化了这个例子；在生产代码中，则应该通过 `panic!` 或 `expect` 来提供关于发生何种错误的更加明确的错误信息。

现在我们有了将注解的 Rust 代码从 `TokenStream` 转换为 `DeriveInput` 实例的代码，让我们来创建在注解类型上实现 `HelloMacro` trait 的代码，如示例 19-33 所示。

文件名: hello_macro_derive/src/lib.rs

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

示例 19-33: 使用解析过的 Rust 代码实现 `HelloMacro` trait

我们得到一个包含以 `ast.ident` 作为注解类型名字（标识符）的 `Ident` 结构体实例。示例 19-32 中的结构体表明当 `impl_hello_macro` 函数运行于示例 19-30 中的代码上时 `ident` 字段的值是 `"Pancakes"`。因此，示例 19-33 中 `name` 变量会包含一个 `Ident` 结构体的实例，当打印时，会是字符串 `"Pancakes"`，也就是示例 19-30 中结构体的名称。

`quote!` 让我们可以编写希望返回的 Rust diamagnetic。`quote!` 宏执行的直接结果并不是编译器所期望的并需要转换为 `TokenStream`。为此需要调用 `into` 方法，它会消费这个中间表示（intermediate representation，IR）并返回所需的 `TokenStream` 类型值。

这个宏也提供了一些非常酷的模板机制；我们可以写 `#name` ，然后 `quote!` 会以 名为 `name` 的变量值来替换它。你甚至可以做些与这个正则宏任务类似的重复事情。查阅 [`quote` crate 的文档](https://docs.rs/quote) 来获取详尽的介绍。

我们期望我们的过程式宏能够为通过 `#name` 获取到的用户注解类型生成 `HelloMacro` trait 的实现。该 trait 的实现有一个函数 `hello_macro` ，其函数体包括了我们期望提供的功能：打印 `Hello, Macro! My name is` 和注解的类型名。

此处所使用的 `stringify!` 为 Rust 内置宏。其接收一个 Rust 表达式，如 `1 + 2` ， 然后在编译时将表达式转换为一个字符串常量，如 `"1 + 2"` 。这与 `format!` 或 `println!` 是不同的，它计算表达式并将结果转换为 `String` 。有一种可能的情况是，所输入的 `#name` 可能是一个需要打印的表达式，因此我们用 `stringify!` 。 `stringify!` 编译时也保留了一份将 `#name` 转换为字符串之后的内存分配。

此时，`cargo build` 应该都能成功编译 `hello_macro` 和 `hello_macro_derive` 。我们将这些 crate 连接到示例 19-38 的代码中来看看过程宏的行为！在 *projects* 目录下用 `cargo new pancakes` 命令新建一个二进制项目。需要将 `hello_macro` 和 `hello_macro_derive` 作为依赖加到 `pancakes` 包的 *Cargo.toml* 文件中去。如果你正将 `hello_macro` 和 `hello_macro_derive` 的版本发布到 [crates.io](https://crates.io/) 上，其应为正规依赖；如果不是，则可以像下面这样将其指定为 `path` 依赖：

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

把示例 19-38 中的代码放在 *src/main.rs* ，然后执行 `cargo run`：其应该打印 `Hello, Macro! My name is Pancakes!`。其包含了该过程宏中 `HelloMacro` trait 的实现，而无需 `pancakes` crate 实现它；`#[derive(HelloMacro)]` 增加了该 trait 实现。

接下来，让我们探索一下其他类型的过程宏与自定义派生宏有何区别。

### 3.2 类属性宏

类属性宏与自定义派生宏相似，不同于为 `derive` 属性生成代码，它们允许你创建新的属性。它们也更为灵活；`derive` 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。作为一个使用类属性宏的例子，可以创建一个名为 `route` 的属性用于注解 web 应用程序框架（web application framework）的函数：

```rust
#[route(GET, "/")]
fn index() {
```

`#[route]` 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

这里有两个 `TokenStream` 类型的参数；第一个用于属性内容本身，也就是 `GET, "/"` 部分。第二个是属性所标记的项，在本例中，是 `fn index() {}` 和剩下的函数体。

除此之外，类属性宏与自定义派生宏工作方式一致：创建 `proc-macro` crate 类型的 crate 并实现希望生成代码的函数！

### 3.3 类函数宏

类函数宏定义看起来像函数调用的宏。类似于 `macro_rules!`，它们比函数更灵活；例如，可以接受未知数量的参数。然而 `macro_rules!` 宏只能使用之前 [“使用 `macro_rules!` 的声明宏用于通用元编程”](https://kaisery.github.io/trpl-zh-cn/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming) 介绍的类匹配的语法定义。类函数宏获取 `TokenStream` 参数，其定义使用 Rust 代码操纵 `TokenStream`，就像另两种过程宏一样。一个类函数宏例子是可以像这样被调用的 `sql!` 宏：

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

这个宏会解析其中的 SQL 语句并检查其是否是句法正确的，这是比 `macro_rules!` 可以做到的更为复杂的处理。`sql!` 宏应该被定义为如此：

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

这类似于自定义派生宏的签名：获取括号中的 token，并返回希望生成的代码。