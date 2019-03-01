# Rust 中的函数式语言功能

[TOC]

## 1. 函数式编程

Rust 吸取了很多其他的语言的优异特点，其中一个就是函数式编程。通常将函数作为参数值或者其他函数的返回值、将函数赋值给变量然后使用（匿名函数）。两个主要的使用场景是闭包（**Closures**）以及迭代器（**Iterators**）。他们具有十分惊人的性能。

## 2. 闭包

Rust 中的闭包与golang中的必报的概念一致，是可以保存进变量或者作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文调用。不同于函数，**闭包允许捕获调用者作用域中的值**。

### 2.1 使用闭包构建行为的抽象

对于一些需要多次调用，每次调用的消耗都很高的函数而言，可以使用闭包来进行优化。如果需要通过一个app根据用户的输出定制锻炼计划，其中涉及了一个消耗很大的计算操作，可以通过以下程序进行模拟：

```rust
pub mod generate {
    use crate::simulated;

    /* Generate the workout plan via an expensive calculation */
    pub fn generate_work_out(intensity: u32, random_num: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated::simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                simulated::simulated_expensive_calculation(intensity)
            );
        } else {
            if random_num == 3 {
                println!("Take a break today!");
            } else {
                println!(
                    "Today, run {} minutes",
                    simulated::simulated_expensive_calculation(intensity)
                );
            }
        }
    }
}

mod simulated {
    use std::time::Duration;
    use std::thread;
    /* Try to simulate an expensive calculation */
    pub fn simulated_expensive_calculation(num: u32) -> u32 {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    }
}
```

其中`generate`module中的函数用以模拟生成训练计划，调用了`simulated`module中的一个极为耗时的计算操作，可以看到，由于多次调用了该函数，使得程序的运行受到很大的影响。为了在计算过程中仅调用该代码一次，去掉多余的两次调用，可以进行以下重构。

#### 使用函数重构

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

现在将该调用的结果存放在一个变量中，时的每次使用这个结果的时候不需要再次计算。但是最大的问题是，即使不需要调用该函数时，也必须执行该计算，这无疑是对于性能的一个负担。

#### 使用闭包进行重构

不同于总是在 `if` 块之前调用 `simulated_expensive_calculation` 函数并储存其结果，我们可以定义一个闭包并将其储存在变量中。然后使用该闭包进行函数的调用。但是仍然需要调用两次，可以在第一次调用后将结果赋值给一个变量，然后可以继续使用，但是还可以使用其他方法。

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

### 2.2 闭包的定义

闭包不要求和函数`fn`一样在参数以及返回值上注明类型。函数需要注明类型，因为他们是暴露给用户的显式接口的一部分。严格的定义这些接口对于保证所有人都认同函数使用以及返回值类型都是很重要的。**但是闭包并不用于这样暴露在外的接口，他们需要存储在变量中使用，不需要命名，也不提供给库函数使用。**

**闭包通常很短，在有限的上下文中使用。**

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;  // 单行代码不需要{}
```

如果尝试调用闭包两次，第一次使用 `String` 类型作为参数而第二次使用 `u32`，则会得到一个错误：

```rust
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```

### 2.3 使用带有泛型以及`Fn` trait 的闭包

对于以上问题的解决方式，可以从以下方面入手：为了减少存储函数调用结果的变量的构建，可以构建一个包含闭包以及执行结果的结构体，该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。这种模式被称为**memorization， lazy evaluation**

为了让结构体使用闭包，需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。每一个闭包都有着自己的独有的匿名类型，也就是说即使两个闭包的函数签名完全一致类型仍然不同。所有的闭包都实现了`Fn`trait，`Fn mut` 或者`FnOnce` .

> 注意：函数也都实现了这三个 `Fn` trait。如果不需要捕获环境中的值，则可以使用实现了 `Fn` trait 的函数而不是闭包。

可以定义一下结构体：

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

`value` 是 `Option<i32>` 类型的。在执行闭包之前，`value` 将是 `None`。如果使用 `Cacher` 的代码请求闭包的结果，这时会执行闭包并将结果储存在 `value` 字段的 `Some` 成员中。接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回存放在 `Some` 成员中的结果。`Cacher` 结构体的字段是私有的，因为我们希望 `Cacher` 管理这些值而不是任由调用代码潜在的直接改变他们。

但是这样的实现有两个主要的问题，一是只可以存储相同的结果，每次的函数调用的参数不同应该返回不同的结果，可以使用一个`std::collections::HashMap`来进行值的保存。另一个问体是，它的应用被限制为接受一个`u32`的值返回一个`u32`值的闭包，可以添加泛型参数获得更为灵活的应用：

```rust
    pub struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        /*Default value of the func call is None */
        pub fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: u32) -> u32 {            
            match self.value.get(&arg) {
                None => {
                    let v = (self.calculation)(arg);
                    self.value.insert(arg, v);
                    v
                },
                Some(&v) => v,
            }
        }
    }
```

### 2.4 闭包会捕获环境

闭包的一大特点是可以捕获其环境中定义的变量：

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 `Fn` trait：

- `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 **环境**，*environment*。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
- `FnMut` 获取可变的借用值所以可以改变其环境
- `Fn` 从其环境获取不可变的借用值

当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 `FnOnce` 。那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 `FnMut` ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 `Fn` 。 在示例 13-12 中，`equal_to_x` 闭包不可变的借用了 `x`（所以 `equal_to_x` 具有 `Fn` trait），因为闭包体只需要读取 `x` 的值。

如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 `move` 关键字。**这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用**。

## 3. 迭代器

Iterator 负责遍历序列中的每一项并决定序列何时结束的逻辑，当时用迭代器时，我们无需重新实现这些逻辑。在 Rust 中，迭代器是**惰性的**（*lazy*），这意味着在调用方法使用迭代器之前它都不会有效果。所以对于一个迭代器对象而言，必须使用一个对应的消耗（**Consume**）方法，才可以对其中的数据进行操作。

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### 3.1 `Iterator` trait and `next` method

迭代器都实现了一个叫做`Iterator` 的trait，它的定义使用了关联类型`Self::Item`, `type Item`.要求实现该trait的同时需要指定一个`Item` 类型，作为`next`方法的返回值。`next`方法是唯一要求被实现的方法，每次返回一个`Some（Item）`, 如果迭代器被消耗完毕，则返回一个None。

**需要注意的是，使用`next`方法返回的是一个不可变引用。**`iter()`方法返回的是一个不可变引用的迭代器，如果需要返回一个获取`v1`所有权，并返回拥有所有权的迭代器，可以使用`into_iter()`方法。`iter_mut()`迭代可变引用。

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

`Iterator` trait 有一系列不同的由标准库提供默认实现的方法；一些方法在其定义中调用了 `next` 方法，这也就是为什么在实现 `Iterator` trait 时要求实现 `next` 方法的原因。这些调用 `next` 方法的方法被称为 **消费适配器**（*consuming adaptors*），因为调用他们会消耗迭代器。

+ `fn size_hint(&self) -> (usize, Option<Usize>)`:

  确定一个迭代器的元素的数量的范围，返回一个元组，第一个数据是下界，第二个是上界，由于可能溢出，所以使用Option

  ```rust
  // The even numbers from zero to ten.
  let iter = (0..10).filter(|x| x % 2 == 0);
  
  // We might iterate from zero to ten times. Knowing that it's five
  // exactly wouldn't be possible without executing filter().
  assert_eq!((0, Some(10)), iter.size_hint());
  
  // Let's add five more numbers with chain()
  let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);
  
  // now both bounds are increased by five
  assert_eq!((5, Some(15)), iter.size_hint());
  ```

+ `fn count(&self) -> usize`

  消耗迭代器，计算元素的数量，一直调用`next`直到返回一个None。使用后该迭代器失效。

+ `fn last(&self) -> Option<Self::Item>`

  消耗迭代器，返回最后一个元素。

+ `fn nth(&self， n: usize) -> Option<Self::Item>`

  消耗迭代器，直到取到下标为n的元素，然后其前面的元素都无法使用。

+ `fn chain<U>(&self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter> where
      U: [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)<Item = Self::[Item](https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item)>,

  将两个迭代器连接在一起。形成一个迭代器。

+ `fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter> where U: IntoIterator`

  将两个迭代器的元素打包为一个迭代器的元素对。

  ```rust
  let s1 = &[1, 2, 3];
  let s2 = &[4, 5, 6];
  
  let mut iter = s1.iter().zip(s2);
  
  assert_eq!(iter.next(), Some((&1, &4)));
  assert_eq!(iter.next(), Some((&2, &5)));
  assert_eq!(iter.next(), Some((&3, &6)));
  assert_eq!(iter.next(), None);
  ```

+ `fn map(self, f: F)`

  使用一个闭包，并且对于迭代器中的每一个元素调用该闭包。

  ```rust
  let a = [1, 2, 3];
  
  let mut iter = a.into_iter().map(|x| 2 * x);
  
  assert_eq!(iter.next(), Some(2));
  assert_eq!(iter.next(), Some(4));
  assert_eq!(iter.next(), Some(6));
  assert_eq!(iter.next(), None);
  ```

+ `fn filter(self, presicate: P) `

  创建一个迭代器，通过使用一个闭包，将所有可以使得闭包返回 true 的元素取出（move），创建一个新的迭代器。

### 3.2 产生不同迭代器的方法

可以使用`Iterator` trait 中的迭代器适配器，将当前的迭代器变为不同类型的迭代器，并且允许使用连式法则调用。

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

```rust
    #[test]
    fn use_other_of_iterator() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

```

### 3.3 自定义类型的迭代器

可以实现 `Iterator` trait 来创建任何我们希望的迭代器。正如之前提到的，定义中唯一要求提供的方法就是 `next` 方法。一旦定义了它，就可以使用所有其他由 `Iterator`trait 提供的拥有默认实现的方法来创建自定义迭代器了！

```rust
    /* Define the personal type to impl the Iterator trait */
    #[derive(PartialEq, Debug)]
    pub struct Pair<'a> {
        id: u32,
        name: &'a str,
        count: u8,
    }

    /* The basic function for the Pair */
    impl<'a> Pair<'a> {
        pub fn new(id: u32, name: &'a str, count: u8) -> Pair {
            Pair {id, name, count}
        }

        pub fn name(&self) -> &str {
            self.name
        }

        pub fn id(&self) -> u32 {
            self.id
        }
    }

    impl<'a> Iterator for Pair<'a> {
        type Item = (u32, &'a str);    // Define the `next()` method return type

        // Use the method of the `Iterator` trait to get the 
        // element tuple for six times
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some((self.id, self.name))
            } else {
                None
            }
        }
    }
```



