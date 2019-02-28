# 集合类型

[TOC]

在Rust中有一系列集合类型，可以包含多个值不同于内建的数组或者元组等数据类型，集合指向的数据是存储在堆上的。也就是说数据的数量可以在运行时确定，不需要在编译时就得知。每种集合类型都有着自己的能力以及代价。

## 1. vector 

`Vec<T>`在一个单独的数据结构中存储同一个类型的多个值，即可变长的数组。关于vectort的结构与golang中的切片类型相似，包含一个头指针，长度，容量.我们希望的设计如下：

```rust
pub struct Vec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}
```

但是由于所有权以及类型问题，该实现是错误的。

由于vector只可以存储同一种类型的值，为了可以存储不同类型的值，可以将这些值定义为一种枚举类型，然后使用该枚举类型的vector进行存储。**Rust在编译时就需要准确得知**

### 1.1 创建vector

可以使用`Vec::new()`方法初始化一个vector，但是注意由于变量的不可变性，以及强类型要求。需要给一个变量指定对应的类型。而且初始化这样一个变量是无法添加元素的。实际使用中更多使用`vec!`这个宏来初始化一个vector。

```rust
let mut v_mut: Vec<i32> = Vec::new();
let v = vec![12, 56, 9];
```

### 1.2 更新 vector

+ `fn push(&mut self, value: T)`:

  将该元素添加到一个vector的末尾。**当元素数量超出usize范围时panic**

+ `fn pop(&mut self) -> Option<T>`:

  删除最后一个元素并且返回，当集合为空，返回None。

+ `fn insert(&mut self, index: usize, element: T)`:

  在特定位置插入元素。将后面的所有元素右移。当index超过vector的长度时panic。

+ `fn remove(&mut self, index: usize) -> T`:

  删除特定位置的元素并且返回。

+ `fn append(&mut self, other: &mut Vec<T>)`

  将vector other中的所有元素添加到该vector中。**并且使得other为空**

+ `fn clear(&mut self)`:
  删除所有元素

+ `fn truncate(&mut self, len: usize)`

  裁剪vector，保留前面len个元素，删除其他的元素。

### 1.3 转化为切片

+ `fn as_slice(&self) -> &[T]` 等价于`&s[:]`
+ `fn as_mut_slice(&self) -> &mut [T]` 等价于 `&mut s[:]`

### 1.4 vector元素访问

可以直接使用索引 `[]` 对于vector进行访问，但是这样会得到变量的所有权。如果只是进行读操作，应该考虑使用引用进行访问。一般具有两种方式：

+ `&v[i]`:返回一个引用类型为`&T`
+ `get(i)`:返回`Option<T>`

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

注意如果使用第一种方式，访问的元素不存在时，程序将会panic.使用第二种方式，不会panic而是返回一个None.这样可以得到比较友好的处理。一旦程序获得了一个有效的引用，借用检查器将会执行所有权以及借用规则，来确保vector的这个引用以及其他引用保持有效。所以以下代码是不允许的：

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

因为一个可变借用以及不可变借用不可以同时存在，或者说该作用域中如果有一个可变引用，那么仅有这一个。

### 1.5 遍历操作  

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

## 2. 字符串

### 2.1 什么是字符串

Rust 倾向于确保暴露所有可能的问题，所以会更涉及有关字符串本质的东西。String 是一个比很多程序员想象中更难的数据结构，再结合 UTF-8，难度较大。在 Rust 中，核心语言中只有一种字符串：`str`， 字符串切片，通常以借用的形式出现，`&str`.字符串切片是存储在别处的 utf-8 编码字符串数据的引用。字符串字面值存储在程序的二进制输出中。

`String`类型是由标准库提供的，没有写进核心语言部分，是可变可增长的，有所有权的，utf-8编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 `String` 和字符串 slice `&str` 类型，而不仅仅是其中之一。

### 2.2 创建字符串

+ `fn new() -> String`:
  创建一个新的空字符串，不分配初始空间，意味着该操作的消耗很小，但是之后添加数据的操作可能造成溢出，如果可以得知大致空间可以使用以下函数。

+ `fn with_capacity(capacity: usize) -> String`

  创建一个规定了初始容量的String

+ `fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>`:
  将一个 bytes 的向量转化为 String，一个字符串切片来自于 bytes（u8），而bytes向量则是由 bytes 组成。可以使用该函数进行转化，但不是所有的 bytes 向量都可以转化为 String，必须满足 utf-8 编码。

  ```rust
  // some bytes, in a vector
  let sparkle_heart = vec![240, 159, 146, 150];
  
  // We know these bytes are valid, so we'll use `unwrap()`.
  let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
  
  assert_eq!("💖", sparkle_heart);
  ```

+ #### `fn from_utf8_lossy(v: &'a [u8]) -> Cow<'a, str>`

  将一个 bytes 的切片转化为 String，包含非法字符。String 由 `u8`组成， bytes 切片来自于 bytes。该函数进行两者间的转化。

+ #### `unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> String`：

  使用指针，容量，长度，创建一个 String

  ```rust
  use std::mem;
  
  unsafe {
      let s = String::from("hello");
      let ptr = s.as_ptr();
      let len = s.len();
      let capacity = s.capacity();
  
      mem::forget(s);
  
      let s = String::from_raw_parts(ptr as *mut _, len, capacity);
  
      assert_eq!(String::from("hello"), s);
  }
  ```

### 2.3 字符串更新

+ #### `fn push(&mut self, ch: char)`

  在一个String的末尾添加一个字符

+ `fn pop(&mut self) -> Option<char>`

  移除最后一个字符并且返回

+ `fn insert(&mut self, idx: usize, ch: char)`

  在一个指定位置添加一个字符，这是一个`O(n)`的操作，因为每一个字符需要复制到 buffer 中去。

+ `fn clear(&mut self)`

  清空一个字符串

+ `fn truncate(&mut self, new_len: usize)`:
  将一个字符串裁剪到指定长度。如果参数中的长度大于当前长度，操作无效。

+ 使用`+`， format 宏拼接字符串

  ```rust
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
  ```

  对于所有`str`类型可以使用的方法，String 类型也可以使用，因为`String`实现了`Deref <Target=str>`,所以可以继承`str`的所有方法，在一个使用`&str`类型的方法中，可以使用`&String`进行调用，被强制转化为`&str`，该方法被称为，**解引用强制多态（deref coercion）**，可以将其理解为，使用了`&s[..]`.

### 3.3 索引字符串

很多语言中，可以通过索引字符串的单个字符进行操作，但是Rust不允许该操作。这就要涉及到`String`的内部表现，由于存储的是 utf-8 编码，所以使用`len（）`方法时，返回的是需要使用的 utf-8 编码的字节数。Rust 不允许使用索引获取 `String` 字符的原因是索引操作预期总是需要常数时间 (O(1))。但是对于 `String` 不可能保证这样的性能，因为 Rust 不得不检查从字符串的开头到索引位置的内容来确定这里有多少有效的字符。由于存储的大小不一致，所以也就没有可能对于String类型进行索引。

```rust
pub fn length_test() {
    let as1 = String::from("Hello");
    let uni1 = String::from("你好啊朋友");  // 中文字符需要三个字节进行编码
    println!("{}, len:{}", as1, as1.len());
    println!("{}, len:{}", uni1, uni1.len());
}
/*Hello, len:5
你好啊朋友, len:15
*/
```

#### 字节，标量值，字形簇

从 Rust 的角度来说，可以从三个方面理解字符串。

字形簇最接近字母的概念，标量值对应 Unicode 编码标量值，字节对应实际存储的字节码。以梵文“नमस्ते”为例：

+ 字节：

  ```
  [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
  224, 165, 135]
  ```

+ 标量值：

  ```
  ['न', 'म', 'स', '्', 'त', 'े']
  ```

+ 字形簇：

  ```
  ["न", "म", "स्", "ते"]
  ```

### 3.4 遍历字符串

对于字符串建立slice通常是一个坏点子，因为字符串索引返回的类型是不明确的：字节值、字符、字形簇、字符串 slice，可能会造成错误的索引，需要谨慎使用。

```rust
let hello = "Здравствуйте";

let s = &hello[0..1];
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
```

可以使用其他获取字符串单个元素的方法，可以使用`bytes(), char()`进行转化：

```rust
pub fn three_format() {
    let data = String::from("नमस्ते");
    //let chars = data.chars();
    //println!("{}, {:?}, {:?}", data, bytes, chars);
    for i in data.chars() {
        print!("{} ", i);
    }
    println!();
    for i in data.bytes() {
        print!("{} ", i);
    }
    println!();

}
```

## 3. Hash Map

与其同语言相似，Hash map是必不可少的一种重要数据结构，与 python中的dict， go中的map 一样都是用于进行`<key, value>`对存储的。与vector不同，HashMap可以使用任意类型的数据作为键值，而不是仅可以使用`usize`。由于HashMap不是那么常用的类型，所以没有将其放入`preclude`包中。需要使用时，需要进行显式引入。

Hash map对于基本类型等实现了`Copy`trait的类型，其值可以拷贝进map，但是对于具有所有权的类型，将其加入map，会发生 move 操作，所有权转移到map中去。	

### 3.1 创建HashMap

+ `HashMap::new()`

  ```rust
  use std::collections::HashMap;
  
  let mut scores = HashMap::new();
  
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  ```

+ 从 vector 元组进行创建

  ```rust
  use std::collections::HashMap;
  
  let teams  = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  
  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect()
  ```

  可以借助元组vector的`collect`方法，其中每个元组包含一个键值对。可以使用`zip（）`方法构建一个元组的vector。这里的`HashMap<_, _>`类型注释是必要的，因为集合类型可能有很多数据结构，需要显式注明。

### 3.2 访问 hash map中的值

可以通过`get()`方法，提供键值获取对应的value。得到的是`Options<T>`类型。可以使用for 循环对于map进行遍历：

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
assert_eq!(map.get(&1), Some(&"a"));
assert_eq!(map.get(&2), None);
```

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 3.3 更新 hash map

可以使用已经存在的键值，赋予新值，覆盖原值。调用`insert`进行更新。可以使用`entry()`方法返回一个枚举类型，只在键值没有被使用时才可以插入。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

