// The recursive constructor
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(s: &str) {
    println!("Just a test: {}", s);
}
use pointer::mybox::CustomPointer;
fn drop_usage() {
    let _a = CustomPointer { data: String::from("Just do it!")};
    let _b = CustomPointer { data: String::from("Try and try")};
    let _c = CustomPointer { data: String::from("The last one")};
    drop(_b);
    println!("Customer pinter created..");
}

fn two_direct_list() {
    use std::rc::Rc;
    use pointer::list::List::{Cons, Nil};

    let a = Rc::new(Cons(12, Rc::new(Cons(123, Rc::new(Nil)))));
    let b = Cons(6, Rc::clone(&a));
    let c = Cons(7, Rc::clone(&a));
    println!("a:{:?}, b:{:?}, c:{:?}", a, b, c);
}

fn rc_clone() {
    use std::rc::Rc;
    use pointer::list::List::{Cons, Nil};

    let a = Rc::new(Cons(123, Rc::new(Cons(124, Rc::new(Nil)))));
    println!("count of the reference: {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count of the reference: {}", Rc::strong_count(&b));
    {
        let _c = Rc::new(Cons(15, Rc::clone(&a)));
        println!("count of the reference: {}", Rc::strong_count(&a));                    
    }
    
    println!("count of the reference: {}", Rc::strong_count(&a));
}

fn main() {
    use List::{Cons, Nil};

    let list = Cons(1, 
                    Box::new( Cons(2, 
                        Box::new( Cons(3, 
                            Box::new(Nil))))));
    println!("{:#?}", list);

    use pointer::mybox::MyBox;

    let x = 123;
    let y = MyBox::new(x);
    println!("The x and MyBox: {}, {:?}", x == *y, y);

    let s = MyBox::new(String::from("LBJ"));
    //hello(*s);
    hello(&s);
    hello(&(*s)[..]);

    drop_usage();
    two_direct_list();
    rc_clone();
}
