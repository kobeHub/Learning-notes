# 测试

[TOC]

## 1. 编写测试的基本操作

 Rust中使用函数进行代码的测试，用以验证非测试代码是否按照期望的方式运行，一般具有以下操作：

+ 设置所需要的数据或者状态
+ 运行测试代码
+ 断言结果是否为所期望的

### 1.1 test 属性的注释函数

Rust 中测试函数是一个带有`test`属性注释的函数。属性attribute是关于Rust代码片段的元数据，`derive`就是一个典型的例子。为了将一个函数设置为测试函数，可以在函数名前加上一个`#[test]`.当执行`cargo test` 时，Rust会构建一个测试程序来调用所有标记了该属性注释的函数。

对于一个成本的测试样例，得到的运行结果：

```rust
running 7 tests
test tests::it_works ... ok
test tests::rectangle_large_can_hold_small ... ok
test tests::greeting_not_done ... FAILED
test tests::length_negative ... ok
test tests::rectangle_small_can_not_hold_large ... ok
test tests::return_result ... FAILED
test tests::width_negative ... ok

failures:

---- tests::greeting_not_done stdout ----
thread 'tests::greeting_not_done' panicked at 'Greeting did not contain the name value:Hello, Cargo', src/lib.rs:40:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- tests::return_result stdout ----
Error: "two add two does not equal three"
thread 'tests::return_result' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', src/libtest/lib.rs:337:5


failures:
    tests::greeting_not_done
    tests::return_result

test result: FAILED. 5 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

输出了所有的失败样例，以及测试结果总结，

通常在一个测试函数中，使用`assert！`类的宏进行测试的第三项工作，该宏接受一个`bool`类型的值，通常使用的宏还有`assert_eq!`, `assert_ne!`,接受两个类型相同的值，断言相等或者不等。**需要注意的是，在一些语言和测试框架中，断言两个值相等的函数的参数叫做 `expected` 和 `actual`，而且指定参数的顺序是很关键的。然而在 Rust 中，他们则叫做 `left` 和 `right`，同时指定期望的值和被测试代码产生的值的顺序并不重要。这个测试中的断言也可以写成 `assert_eq!(add_two(2), 4)`，这时失败信息会变成 `assertion failed: `(left == right)`` 其中 `left` 是 `5` 而 `right` 是 `4`。**

这两个宏底层实现使用了`==  !=` ， 同时如果断言失败会使用`Debug`模式输出错误信息。所以传入的类型必须是实现了`PartialEq`以及`Debug` trait 的类型。又因为这两个trait都是派生trait，通常可以在结构体上加`#[derive(PartialEq + Debug)]`属性注释。

### 1.2 自定义失败信息

可以向`assert！`等宏中添加一个可选的默认失败信息，在测试失败时将这些信息打印出来。获取更加详细的测试信息。

例如，对于一个尚未完成的函数进行测试，由于可能`Hello`可变，只对参数进行测试：

```rust
# fn main() {}
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

如果测试失败，显示的结果只有失败的函数以及行号：

```shell
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
        thread 'tests::greeting_contains_name' panicked at 'assertion failed:
result.contains("Carol")', src/lib.rs:12:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```

可以添加更多信息：

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

### 1.3 panic 测试

对于代码中的panic进行测试时，可以使用`should_panic`属性注释进行操作：

```rust
#[test]
#[should_panic(expected = "The incorrect value of rectangle: 12.1, -5")]
fn width_negative() {
        Rectangle::new(12.1, -5.);
}

impl Rectangle {
        pub fn new(length: f64, width: f64) -> Rectangle {
            if length <= 0. || width <= 0. {
                panic!("The incorrect value of rectangle: {}, {}", length, width);
            }
            Rectangle {
                length,
                width,
            }
        }
}

```

但是使用这种方式有一个很大的问题，如果是程序的其他部分panic那么该测试样例也是可以通过的，是的测试结果十分含糊不清，所以一般需要加上一个`expeted`参数，从而获取所期望的panic信息。

### 1.4 用`Result<T, E>`进行测试

可以在测试函数返回一个`Result<T, E>`枚举类型，可以达到同样的效果：

```rust
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

这里我们将 `it_works` 改为返回 Result。同时在函数体中，在成功时返回 `Ok(())` 而不是 `assert_eq!`，而失败时返回带有 `String` 的 `Err`。跟之前一样，这个测试可能成功或失败，不过不再通过 panic，可以通过 `Result<T, E>` 来判断结果。为此不能在对这些函数使用 `#[should_panic]`；而是应该返回 `Err`！  