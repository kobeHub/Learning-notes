# 不安全 Rust

Rust 得益于强大的类型系统以及借用规则，在编译时强制执行内存安全保证。除此之外，Rust还隐藏了第二种语言，它不会强制执行这类内存安全保证，具有更强大的力量。不安全Rust之所以存在，是因为静态分析始终是保守的，为了安全的内存使用往往会拒绝一些代码的编译。除此之外，底层计算机硬件固有的不安全性，如果Rust不允许不安全操作，很多任务根本完成不了。

使用`unsafe`关键字可以切换到不安全Rust， 有以下四类“超级力量”：

+ 解引用**裸指针**
+ 调用不安全的函数或者方法
+ 访问或修改可变静态变量
+ 实现不安全的`trait`
+ 访问`union`的字段

**`unsafe`不会关闭借用检查器或者禁用Rust任何其他的安全检查**，如果在不安全代码中使用引用，它仍会被检查。`unsafe` 关键字只是提供了那四个不会被编译器检查内存安全的功能。你仍然能在不安全块中获得某种程度的安全。

为了尽可能隔离不安全代码，将不安全代码封装进一个安全的抽象并提供安全 API 是一个好主意。标准库的一部分被实现为在被评审过的不安全代码之上的安全抽象。这个计数防止了 `unsafe` 泄露到所有你或者用户希望使用由 `unsafe` 代码实现的功能的地方，因为使用其安全抽象是安全的。

## 1. 解引用裸指针

unsafe Rust有两个被称为**裸指针(Raw Pointer)** 的类似于引用的类型：`*const T`, `*mut T` 分别代表不可变裸指针以及可变裸指针。裸指针与引用和智能指针的区别在于；

+ 允许忽略借用规则，可以同时拥有不可变指针、可变指针；或者多个指向相同位置的可变指针
+ 不保证指向有效内存
+ 允许为空
+ 不能实现任何自动清理功能

```Rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

可以在安全代码中创建裸指针，但是必须在不安全代码中解引用裸指针。使用裸指针大大增加数据竞争的风险，主要的应用场景是调用C接口，此外还可以构建编译器无法理解的安全抽象。

## 2. 调用不安全的函数或者方法

第二类要求使用不安全块的操作是调用不安全函数。不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 `unsafe`。`unsafe` 表明我们作为程序需要满足其要求，因为 Rust 不会保证满足这些要求。通过在 `unsafe` 块中调用不安全函数，我们表明已经阅读过此函数的文档并对其是否满足函数自身的契约负责。

### 2.1 创建不安全代码的安全抽象

将不安全代码封装进安全的函数是一个常见的抽象，以标准库中的`spli_as_mut()`函数为例，该函数接受一个可变slice，返回两个可变slice，根据Rust的借用规则，编译器不允许同时存在两个相同数据的可便借用，需要涉及到unsafe函数的调用:

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

slice是一个指向一些数据的指针，并带有slice的长度。使用`as_mut_ptr`方法获取slice的裸指针。`slice::from_raw_parts_mut`是一个不安全的函数调用，需要获取一个裸指针，组建新的slice。用于指针运算的`offset`函数也是不安全的，所以都位于`unsafe`块中。

### 2.2 使用`extern`函数调用外部代码

可以使用`extern`关键字创建使用**外部函数接口（Foreign Funtion Interface， FFI）**。`extern`块中生命的代码总是不安全的，因为其他语言无法强制执行Rust规则而且Rust无法进行检查。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

在`extern “C”` 块中，列出了我们希望调用的另一个语言中的外部函数的签名以及名称。`"C"` 部分定义了外部函数所使用的 **应用程序接口**（*application binary interface*，ABI） —— ABI 定义了如何在汇编语言层面调用此函数。`"C"` ABI 是最常见的，并遵循 C 编程语言的 ABI。

**从其他语言调用Rust函数**

使用`extern`创建允许其他语言调用的Rust接口。不同于 `extern` 块，就在 `fn` 关键字之前增加 `extern` 关键字并指定所用到的 ABI。还需增加 `#[no_mangle]` 注解来告诉 Rust 编译器不要 mangle 此函数的名称。*Mangling* 发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，不过会使其名称更难以阅读。每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust fn");
}
```

### 2.3 访问或修改可变静态变量

Rust 支持全局变量，可以通过`static`关键字进行定义。对于可变静态变量的方外以及修改都是`unsafe`的。不可变静态变量与常亮之间的区别:

+ 静态变量必须标注变量类型
+ 静态变量只能存储`‘static` 生命周期的变量
+ 静态变量的值具有固定的内存地址，常亮则允许任意的复制

### 2.4 实现不安全trait

最后一个只能用在 `unsafe` 中的操作是实现不安全 trait。当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的。可以在 `trait` 之前增加 `unsafe` 关键字将 trait 声明为 `unsafe`，同时 trait 的实现也必须标记为 `unsafe`，

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```