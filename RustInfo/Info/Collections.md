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

