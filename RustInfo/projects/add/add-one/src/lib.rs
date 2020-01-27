#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(add_one(2), 3);
    }
}

pub fn add_one(x: i32) -> i32{
    x + 1
}

/// 递归类型，`cons List` 在编译时无法得知所需分配的空间大小
/// Rust 无法计算要为定义递归类型分配多少空间，可以存储一个指向
/// 值的指针，使用`Box`, `Rc`, `&`
#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use std::fmt::Display;

/// 递归类型，使用泛型以及生命周期
#[derive(Debug)]
pub enum Listt<'a, T>
where T: Display {
    Ele(T, &'a str, Box<Listt<'a, T>>),
    Non,
}
