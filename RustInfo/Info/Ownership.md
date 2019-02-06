# Rust 中的所有权

[TOC]

## 1. 所有权的含义

所有权系统（**Ownership**）使得Rust无需进行垃圾回收（Garbage Collector），不需要显式进行内存管理。实现自动的内存管理的机制。所有程序都必须管理其使用的计算机内存，rust使用所有权系统进行内存管理。编译时，按照一定的规则进行检查，运行时所有权系统的任何功能都不会减慢程序。

> **Stack and Heap:**
>
> 堆和栈都是代码运行时可供使用的内存，但是具有不同的结构。**Stack(LIFO)**，栈中的数据都必须占用已知而且固定的大小，同时由于数据的存储总是在栈顶，所以不需要寻找数据存取的位置，使得栈的操作十分快速。
>
> 堆主要用于处理编译时大小未知或者大小可能变化的数据，主要用以处理运行时的数据。堆是缺乏组织的：向堆中放入数据时需要请求一定大小的空间，操作系统找到足够大的空间后，将其标记为已使用，并且返回表示该位置的指针（pointer）。该过程被称之为**在堆上分配内存**，简称为分配。将数据入栈并不认为是分配，因为指针的大小是固定的，可以将指针存储在栈上，但是实际需要使用数据指针时必须访问指针。
>
> 使用堆上的数据显然要比栈上慢很多，因为需要使用指针进行跳转访问。现代处理器中内存跳转越少，访问速度就越快。
>
> 对于基本的数据类型，例如：整形，浮点型，布尔，char都是存储在栈上的，离开作用域即被移出栈。对于可变大小的数据类型，例如`std::string::String`等较为复杂的类型，使用堆进行分配。

**跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。**

### 1.1 所有权的规则

+ Rust中每一个值都有一个被称为其所有者**(Owner)**的变量
+ 每一个值有且仅有一个所有者
+ 当所有者离开作用域后，该值被丢弃

### 1.2 变量作用域

作用域是一个项（**item**）在程序中有效的范围。通常在一个一个代码块中，一个项是有效的被成为其作用域。

对于字符串类型，**为了提高执行的效率**可以分为字符串字面值`str`，以及字符串`String`。对于字符串字面值来说，在编译时就知道其内容以及数据大小。所以文本被直接硬编码进入最终的可执行文件中去，这使得字符串字面值快速而且高效，使用`let`语句声明一个固定的字符串，默认就是字符串字面值。

对于`String`类型，为了支持一个可变长的文本片段，需要在堆上分配一个编译时未知大小的内存进行内容的存放。这就需要满足以下两点：

+ 必须在运行时向操作系统申请内存
+ 需要一个处理完string时将内存返回给操作系统的方法

```rust
/* Test for str and string*/
pub fn str_string() {
    let a = "It's a str";
    let b = String::from("It's a string");
    let () = a;
    let () = b;
}
/*error[E0308]: mismatched types                                                  
 --> src/string/str_string.rs:5:9                                               
  |                                                                             
5 |     let () = a;                                                             
  |         ^^ expected str, found ()                                           
  |                                                                             
  = note: expected type `str`                                                   
             found type `()`                                                    
                                                                                
error[E0308]: mismatched types                                                  
 --> src/string/str_string.rs:6:9                                               
  |                                                                             
6 |     let () = b;                                                             
  |         ^^ expected struct `std::string::String`, found ()                  
  |                                                                             
  = note: expected type `std::string::String`                                   
             found type `()`                  */
```

使用内部模块时，可以创建相应的模块文件夹，然后创建`mod.rs`文件，使用`pub mod module_name`将需要外部使用的`pub`函数或者方法，导出。注意使用是需要使用`mod crate::module::part`,将每一个小模块模块化，然后可以调用其中的函数。

关于`String`的构造，可以使用三种方式从一个`str`构造一个`String`。关于释放内存，要求每一个`allocate`对应一个`free`。

```rust
pub fn string_test() {
    println!("Just a simple usage of String");
    println!("3 methods to initialize a String:");
    println!("str.to_string(), String.from(str), str.into()");
    let origin = "Hello, it's the origin str";
    let mut str1 = String::from(origin);
    let str2: String = origin.into();
    let str3 = origin.to_string();

    str1.push_str(", and the mutable str is me!");
    println!("{}, {}, {}", str1, str2, str3);
}
/*Just a simple usage of String
3 methods to initialize a String:
str.to_string(), String.from(str), str.into()
Hello, it's the origin str, and the mutable str is me!, Hello, it's the origin str, Hello, it's the origin str
*/
```

关于内存的释放，Rust使用了不同的策略，当变量离开作用域时，其拥有的内存内存就被释放。rust使用了一个特殊的函数，当一个变量离开作用域时，使用`drop`函数（在c++中这种item在生命周期结束时释放资源的模式叫做**资源获取即初始化(Resource Acquisition Is Initialization)**）。 

### 1.3  移动

对于堆上使用的数据，进行复制时，使用的是**移动（move）**操作，与常规的深复制或者浅复制不同。为了节省空间，相当于进行了变量替换。

```rust
let s1 = String::from("hello");
let s2 = s1;
/*error[E0382]: borrow of moved value: `str1`                                     
  --> src/string/str_string.rs:33:38                                            
   |                                                                            
32 |     let str2 = str1;                                                       
   |                ---- value moved here                                       
33 |     println!("{}, {}, {}, {}", x, y, str1, str2);                          
   |                                      ^^^^ value borrowed here after move   
   |                                                                            
   = note: move occurs because `str1` has type `std::string::String`, which does not implement the `Copy` trait
*/
```

String类型存储了三个数据，头指针，长度，容量。将s1赋值给s2，相当于将s1的三个属性全部给了s2，但是实际上的字符串数据并不复制。但是`s1`已经不可以继续使用。使用移动而不是用浅赋值，是因为在一个作用域结束后，如果根据所有权归责，两个指向同一块内存的变量将会使用两次`drop（）`，出现了多次释放内存，这是不合法的。

**这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。**

### 1.4 clone

也可以使用堆上数据的深复制**克隆**，为新的变量分配新的内存空间。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### 1.5 拷贝

只发生在栈上的复制，也就是基本类型的复制，是深复制，没有发生移动。因为基本类型这样的编译时就知道大小的类型存储在栈上，进行拷贝是快速的。使用的是`Copy`trait。如果一个类型具有`Copy`trait，一个旧的变量在赋值给其他变量后，仍然可以使用。

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

- 所有整数类型，比如 `u32`。
- 布尔类型，`bool`，它的值是 `true` 和 `false`。
- 所有浮点数类型，比如 `f64`。
- 字符类型，`char`。
- 元组，当且仅当其包含的类型也都是 `Copy` 的时候。比如，`(i32, i32)` 是 `Copy` 的，但 `(i32, String)` 就不是。

## 2. 所有权与函数

将一个值传递给函数，在语义上与变量赋值相似，所以将一个变量作为参数传入或者作为返回值都会伴随着变量所有权的交换。

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

注意一个变量的所有权如果不在其作用域中进行转移，那么在作用域结束后，该变量将不可用。所以在计算一个string长度的函数中，为了使得变量可用，必须采用以下操作。

```rust
// return the ownership of string by return string
pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

## 3. 引用

为了避免以上问题，可以使用**引用**。使用变量的引用不获取其所有权，所以变量在之后还可以继续使用。

![](https://rustlang-cn.org/assets/img/trpl04-05.4b14ca48.svg)

`s = &s1`,使用引用操作，可以使用对应值不获取其所有权。引用的对应操作是解引用`*`，使用引用并不获得对应的值，所以在引用离开作用域时，对应的值并不会消失。通常将一个变量的引用用于函数的参数，避免多余的所有权转移操作。将获取引用作为函数的参数称为**借用（borrowing）**。

注意，由于变量的默认不可变性，如果需要改变引用的值，需要使用可变引用。`&mut type`.

```rust
// mutable reference to modify the value
pub fn append(some_string: &mut String) {
    some_string.push_str(", Rustean");
}

str_string::append(&mut s1);  // mut s1
println!("After append:{}", s1)

```

注意使用可变引用的前提是变量是可变的。**注意可变引用有一个很大的限制，在特定的作用域中对于一个特定的数据，有且仅有一个可变引用，使用唯一的可变引用，可以避免可能的数据多处修改，从而避免了数据竞争。**数据竞争类似于竞争条件。需要满足以下条件：

+ 两个或者多个指针同时访问同一数据
+ 至少一个指针试图写入数据
+ 没有同步的数据访问机制

**同时不可以在拥有不可变引用的同时，使用可变引用，因为不可变引用用户不期望数据发生变化。**

```rust
error[E0499]: cannot borrow `s` as mutable more than once at a time             
  --> src/string/str_string.rs:42:14                                            
   |                                                                            
41 |     let s1 = &mut s;                                                       
   |              ------ first mutable borrow occurs here                       
42 |     let s2 = &mut s;                                                       
   |              ^^^^^^ second mutable borrow occurs here                      
43 |     println!("{}{}", s1, s2);                                              
   |                      -- first borrow later used here              
```

## 4. 垂直引用

在具有指针的语言中，很容易出现垂直指针，也就是指向已经释放内存的变量的指针。但是Rust中的引用永远不会出现垂直引用，会在编译时报错。

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
/*error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime*/
```

