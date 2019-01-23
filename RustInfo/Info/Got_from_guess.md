# Got from guessing game

[TOC]

## 1. 使用cargo构建项目

Rustacean们使用cargo进行项目管理，使用cargo可以进行构建代码、下载依赖、编译依赖等任务。可以使用`cargo new project_name`命令来新建一个项目。新建该项目后，生成存放该项目的文件夹，并且生成了相应的项目配置文件`Cargo.toml`,首次运行`cargo build`时在项目的根目录下会生成一个`Cargo.lock`用以记录项目的真实依赖，该文件不需要手动操作，一切都有cargo自动完成。

### 1.1 构建以及运行

可以使用以下命令对于cargo项目进行构建以及运行测试：

```shell
cargo build
./target/debug/guessing_game
```

也可以使用一个命令完成编译以及测试的任务：

```
cargo run
```

cargo提供了一个快速检验代码的功能，不需要产生可执行文件，快速检查代码确保其可以通过编译。

```shell
cargo run
```

+ 使用`cargo check`检查项目
+ 使用`cargo run`进行一键测试

### 2.2 发布版本

当项目调试完毕，可以使用`cargo build --release`来优化编译项目，会在`target/release`目录下生成可执行文件，优化后的代码运行更快，但是需要更多的编译时间。所以设计两种不同的编译模式，一种适用于生产环境的测试，另一种适用于最终的产品发布。所以在测试代码的运行时间需要使用`release`版本，会有更快的运行速度。

## 2. 项目的配置文件

当新建一个项目时，在项目的根目录下会新建一个配置文件，里面包含了项目的基本信息，以及所使用的依赖和依赖的版本号。在目录`src`下进行代码的实现，进行快速迭代时可以使用`cargo RUN`进行快速的运行测试。`src/main.rs`是项目的入口源文件。

将所有的依赖写在`Cargo.toml`：

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Inno Jia <jdgets111@gmail.com>"]
edition = "2018"

[dependencies]
rand  = "0.3.14"
```

## 3. 实现细节

### 3.1 引入库

使用关键字`use`引入需要使用的库文件，rust有一个标准库`std`。rust标准库是便携式rust软件的基础。这是一套针对广泛的rust生态系统设计的最小的经过实战考验的共享的抽象库。提供了核心的类型，语言基元的库定义操作，标准宏，I/O,多线程等。

以进行输入输出为例，需要引入`use std::io` 

默认情形下Rust将`prelude`模块中的少量类型引入到每个程序的作用域，如果需要使用的类型不在prelude中，必须进行手动的引入。其中`prelude`是预引入的模块，因为标准库十分庞杂，对于一个作用域需要使用的类型如果全部手动引入会十分麻烦，但是引入过多没有用到的库会造成代码冗余。所以使用了预引入模块，预先引入尽可能少的库，包含每一个程序都可能使用到的内容。

### 3.2 程序入口

`main.rs`文件中的`fn main（）{}`函数

### 3.3 使用变量

Rust中使用关键字`let`定义一个变量，由于Rust是静态类型的语言，对于每一个变量的声明都需要指明其类型，然后可以把一个初始化后的对象赋给变量，或者变量间的赋值，这与基本的静态类型语言一致。

```rust
let mut guess = String::new(); // String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块。
let foo = bar;
```

在Rust中变量默认是不可变类型，可以在变量声明时，在其前面加上`mut`关键字使之成为可变类型。

在Rust中使用一个类型的**关联函数**时需要借助`::`操作符，关联函数是针对于某个特定的类型实现的，而不是针对于某个对象，叫做静态方法，只可以使用类型加上`::`进行调用，这点与java相同；如果关联函数针对某一个特定的实例实现，可以通过实例`.`的方式进行调用。

```rust
io::stdin()::read_line(&mut guess)
    .expect("Failed to read line")
```

使用`io`类型的`stdin()` 函数返回了一个标准输入的实例`std::io::Stdin` ,可以使用该实例进行终端的输入。然后调用该实例的`read_line()`方法，将用户输入存入到一个字符串中，该字符串以参数的形式传入其中，其参数类型是一个引用，可以对原字符串的内容进行更改，多处代码访问同一个数据而不需要在内存中多次赋值，与c语言一致，**但是可变变量的使用需要加上mut关键字**。

对于一个较长行可以在调用函数处换行，每一行使用`；`分隔。当调用`ready_line(&)`时返回一个`io::Result` 值。`Result`的类型是枚举，enums。枚举类型与c中的一致，包含了特定集合的值，`Ok, Err`, 分别代表操作是否成功。`expect()`方法如果接收到的是`Err`会使得程序终止，并且输出错误信息。

如果不使用`expect()`,程序编译时会发出警告，没有使用`io::sidin()::read_line(&mut guess)`的返回值。

### 3.4 引入包crate

在rust中包的概念使用crate表示，相当于其他语言中的package，library。一个crate中包含多个module。对于cargo项目如果需要引入一个包，可以在配置文件中的依赖性下进行添加。

`rand` crate是一个生成随机数的包，然后使用`cargo build`重新构建项目，此时carfo检查依赖发现本地没有，从registry上获取所有包的最新版本信息。然后检查`【dependencies】`片段并且下载缺失的crate,下载过程中会递归下载所有的依赖包。

Cargo 有一个机制来确保任何人在任何时候重新构建代码，都会产生相同的结果：Cargo 只会使用你指定的依赖版本，除非你又手动指定了别的。例如，如果下周 `rand` crate 的 `0.3.15` 版本出来了，它修复了一个重要的 bug，同时也含有一个会破坏代码运行的缺陷，这时会发生什么呢？

这个问题的答案是 *Cargo.lock* 文件。它在第一次运行 `cargo build` 时创建，并放在 *guessing_game* 目录。当第一次构建项目时，Cargo 计算出所有符合要求的依赖版本并写入 *Cargo.lock* 文件。当将来构建项目时，Cargo 会发现 *Cargo.lock* 已存在并使用其中指定的版本，而不是再次计算所有的版本。这使得你拥有了一个自动化的可重现的构建。换句话说，项目会持续使用 `0.3.14` 直到你显式升级。

**更新crate**

可以使用`cargo update`进行包更新，但是进行的是小版本更新，如果大版本更新，必须手动更改配置文件

### 3.5 生成随机数

引入随机数包后需要在项目中使用,`use rand::Rng;` Rng是一个`trait`可以认为是其他语言中的接口，定义了随机数生成器应实现的方法，这个trait必须在作用域中。`Rng`定义了`gen_range`方法，通过使用`rand::thread_rng()`静态方法实例化一个随机数生成器，然后该生成器实现了`Rng`trait，所以可以调用该方法生成一个指定范围的随机整数。`rand::thread_rng` 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。

```rust
let num = rand::thread_rng().gen_range(1, 101)
```

对于一个trait定义的方法，可以使用`cargo doc --open`在本地查看。

### 3.6 数字比较

可以引入标准库中的`std::cmp::Ordering` 类型进行二值比较操作，该类型也是一个枚举类型，成员：

```rust
Less, Greater, Equal
```

可以使用`match`表达式对于比较结果进行操作，mathc表达式有多个分支组成，一个分支包含一个模式（pattern）.

```rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
}
```

但是需要注意的是，rust语言是一个静态强类型语言，对于不同类型间的比较必须进行显式的类型转换。`guess`是String，`secret_num`是数字类型，所以必须把String转化为数字类型。

```rust
let guess: i32 = guess.trim().parse()
	.expect("It's not a number");
```

在这里声明了一个影子变量来覆盖原有的变量，通常用于需要类型转换的情景，而不是创建两个变量。对于字符串，先使用`trim（）`减去空白字符，然后使用`parse()`方法转换为`i32`类型。与之前的输入操作一样，该操作也易发生错误，所以返回一个`Result`变量，需要使用`expect()`函数进行处理。

为了让输入错误后程序继续执行，可以将expect函数转换为进一步的处理:将 `expect` 调用换成 `match` 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。须知 `parse`返回一个 `Result` 类型，而 `Result` 是一个拥有 `Ok` 或 `Err` 成员的枚举。这里使用的 `match` 表达式，和之前处理 `cmp` 方法返回 `Ordering` 时用的一样。

```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```



### 3.7 使用循环

`loop`关键字创建一个无限循环，需要手动设置循环终止条件。

### 3.8 通配符 

`_`可以用来指代一个不需要使用的变量