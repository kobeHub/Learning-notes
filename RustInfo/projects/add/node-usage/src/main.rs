use node::Node;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf count: strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf count: strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
        println!("branch count: strong = {}, weak = {}",
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch));
    }

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("leaf count: strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

}
