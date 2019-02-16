# Rust 中的错误处理

[TOC]

## 1. panic 与不可恢复错误

与 golang 相似，可以使用`panic！`调用，产生一个不可恢复的错误。当时用这个宏时，Rust 会打印一个错误信息，展开并清理栈数据。基本的使用场景是检测一些类型的 bug， 并且无法处理。

> **对应panic的栈展开或者终止**
>
> 当出现一个panic时，默认对于栈进行展开（unwinding），将会回溯站并且清理遇到的每一个函数的数据。这种回溯需要进行很多的工作。另一种选择是进行终止，不清理数据直接退出。此时程序拽用的内存有操作系统清理。可以在`Cargo.toml`中添加	模式：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

### 使用`panic！`的 backtrace

可以在运行时，添加`RUST_BACKTRACE=1`的环境变量，查看发生panic时的函数调用栈。

## 2. Result 与可恢复错误

可以使用一个枚举类型来处理潜在的错误，`Result`类型包含两个成员，定义如下：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

其中`T` 和`E`是泛型类型参数，可以借助泛型将`Result`类型用于很多场景。