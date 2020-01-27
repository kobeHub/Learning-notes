//! # my-box
//!
//! A library of smart pointer, define the consumed `Box`
//! type.


/// The basic box struct, use tuple struct
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    /// assoicated type
    type Target = T;

    /// When use `*y` actually call `*(deref(y))`
    fn deref(&self) -> &T {
        &self.0
    }
}

/// Customer smart pointer
pub struct CSPointer {
    data: String
}

impl CSPointer {
    pub fn new(s: &str) -> CSPointer{
        CSPointer{
            data: String::from(s)
        }
    }
}

impl Drop for CSPointer {
    fn drop(&mut self) {
        println!("Dropping data: {}", self.data);
    }
}

use std::rc::Rc;
/// Recursive List based on `Rc`
#[derive(Debug)]
pub enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

use std::cell::RefCell;
/// Use `Rc` and `RefCell` get a multiple owner and mutable
/// object
#[derive(Debug)]
pub enum RcRefList<T> {
    Cons(Rc<RefCell<T>>, Rc<RcRefList<T>>),
    Nil,
}

/// Use `RefCell`, `Rc` build a dome to show
/// references cycles
#[derive(Debug)]
pub enum RefRcList<T> {
    Cons(T, RefCell<Rc<RefRcList<T>>>),
    Nil,
}

impl<T> RefRcList<T> {
    pub fn tail(&self) -> Option<&RefCell<Rc<RefRcList<T>>>> {
        match self {
            RefRcList::Cons(_, item) => Some(item),
            RefRcList::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyBox;
    use std::format;

    #[test]
    fn mybox_usage() {
       // fn hello(x: &str) -> String {
       //     format!("Hello, {}", x)
        // }
        let hello = |x: &str| format!("Hello, {}", x);
        let m = MyBox::new(String::from("Jack"));

        // 对于`hello`函数的调用:
        // 便一起发现类型不一致，并且MyBox实现了Deref trait
        // 所以调用deref方法，得到&String, 标准库中提供了 String Deref trait
        // 实现，再次调用deref方法，返回字符串slice，即&str, 即可调用hello
        assert_eq!(String::from("Hello, Jack"), hello(&m));
        assert_eq!(String::from("Hello, Jack"), hello(&(*m)[..]));
    }
}
