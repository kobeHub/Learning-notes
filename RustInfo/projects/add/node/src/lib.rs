//! # node
//!
//! A library to show how to use `Weak<T>` to avoid
//! references cycles. `Weak<T>` has no owner.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A Tree data structure with a vale and Node
/// `Node` own its `children`, in order to access
/// each node of the tree via variables, define
/// `Vec<Node>` as `Rc<Vec<Node>>`. Use `RefCell<T>`
/// to make children mutable.
///
/// 为了增加从子节点到父节点的引用，父节点应该拥有其
/// 子节点，但是子节点不应该拥有其父节点
/// 即： 父节点丢弃了，子节点也应共丢弃，子节点丢弃了，
/// 父节点不该被丢弃
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::rc::{Rc, Weak};
    //use std::cell::RefCell;

    #[test]
    fn it_should_avoid_references_cycles() {
        let leaf = Rc::new(Node{
            value: 1234,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        assert!(leaf.parent.borrow().upgrade().is_none());

        let branch = Rc::new(Node{
            value: 123,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        assert_eq!(123, leaf.parent.borrow().upgrade().unwrap().value);
    }
}
