# 包、crate、模块

[TOC]

## 1. 作用域

在编写程序时，一个核心的问题是作用域（scope）.在Rust中有关作用域的概念可以使用package、crate、modules、path进行管理。

+ 包：允许构建、测试以及分享crate
+ crate：一个模块的树形结构，作为库或者二进制项目的结构
+ 模块：可以使用use关键字导入其他模块，可以控制作用域以及模块的私有性
+ 路径：一个指向函数，结构体，模块的命名

### 1.1 项目的异同

**crate**可以是一个二进制项目或者一个库项目，主要区别在于在`src`文件夹下的`main.rs`,`lib.rs`Cargo 的约定是如果在代表包的 *Cargo.toml* 的同级目录下包含 *src* 目录且其中包含 *main.rs* 文件的话，Cargo 就知道这个包带有一个与包同名的二进制 crate，且 *src/main.rs* 就是 crate 根。另一个约定如果包目录中包含 *src/lib.rs*，则包带有与其同名的库 crate，且 *src/lib.rs* 是 crate 根。crate 根文件将由 Cargo 传递给 `rustc` 来实际构建库或者二进制项目。

一个包中至少包含一个二进制crate或者库crate，可以包含多个二进制crate，至多一个库crate。

## 2. 模块

可以使用关键字`mod`定义一个模块，可以在同一个文件中定义不同的模块，也可以在多个文件中进行模块的定义。**包和crate用来构建二进制项目，`src/main.rs`, `src/lib.rs`用来作为crate根**。从该根出发，以类似文件目录的形式逐层包含不同的模块。

```rust
crate
└── sound
    ├── instrument
    │   └── woodwind
    └── voice
```

### 2.1 私有性

在Rust中木块作为私有性边界（privacy boundary）。关于私有性的规则如下：

+ 所有项（函数，方法，结构体，枚举类型，模块，变量）默认都是私有的
+ 可以使用`pub`声明共有项
+ 不允许使用定由于当前模块子模块的私有代码
+ 允许使用任何定义于父模块或者当前模块的代码

也就是说，对于子模块中的不含有`pub`的项都不可以使用，但是对于所有服模块中的项都来可以直接使用。

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // 函数体
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

### 2.2 结构体以及枚举可见性

对于结构体而言，可以将结构体公有，在没一个字段的基准上也可以选择是否公有；对于枚举类型而言，如果将一个枚举类型声明为公有，那么所有的字段都是公有的。

## 3. 路径

路径用来引用模块树中的项。可以使用绝对路径或者相对路径引用模块中的内容，使用分隔符`::`隔开。

+ 绝对路径：从crate根开始的路径
+ 相对路径：使用`self`获取当前路径，`super`获取上一级路径

```rust
# fn main() {}
#
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // 函数体
}
```

### 3.1 use 引入的基本规则

使用`use`将其他模块引入当前作用域时，一般遵守以下规则。

+ 结构体、枚举类型，一般引入全路径，也就是在当前路径可以直接使用该类型
+ 函数或者方法一般引入父路径，使用`super_name::func`的语法进行调用

但是注意如果将不同模块的同名项引入作用域是不允许的。只能引入到模块。为了解决同名项的问题可以使用`as`关键字，将引入的项进行更名。

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
#     Ok(())
}
fn function2() -> IoResult<()> {
#     Ok(())
}
```

注意使用`use`关键字将一些项引入当前作用域时，这些项默认都是私有的。如果希望调用编写的代码的代码能够像你一样在其自己的作用域内引用这些类型，可以结合 `pub` 和 `use`。这个技术被称为 “重导出”（*re-exporting*），因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // 函数体
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
```

### 3.2 使用外部包

在cargo项目中如果需要使用外部包，可以在`cargo.toml`中添加依赖，需要使用`=`指明以来的版本。告诉cargo需要下载的依赖。然后就可以使用这些外部包。

### 3.3 嵌套路径消除多个use

可以使用嵌套路径，将多个属于同一个模块的项引入到当前作用域。

```rust
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```

如果需要将一个模块中的所有公有项全部引入当前路径，可以使用`use mod1::mod2::*;`但是不推荐使用，因为如此难以推导作用域中有什么名称和它们是在何处定义的。glob 运算符经常用于测试模块 `tests` 中，这时会将所有内容引入作用域。

 