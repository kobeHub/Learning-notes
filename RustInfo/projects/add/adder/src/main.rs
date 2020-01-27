use add_one;

use my_box::CSPointer;

fn drop_usage() {
    let _a = CSPointer::new("first");
    let _b = CSPointer::new("second");
    println!("Customer smart pointer created.");
}

fn force_drop() {
    // use std::mem::drop;

    let a = CSPointer::new("first");
    println!("Customer smart pointer created,");
    drop(a);
    let _b = CSPointer::new("second");
    println!("Customer smart pointer 1 drop before the main end");
}

fn main() {
    let n = 10;
    println!("use add_one: {}", add_one::add_one(n));
    {
        use add_one::List::{Cons, Nil};
        use add_one::Listt::{Ele, Non};

        println!("Use Box to recursively construct List");
        let list = Cons(1,
                        Box::new(Cons(2,
                                      Box::new(Cons(3,
                                                    Box::new(Cons(4,
                                                                  Box::new(Nil))))))));
        println!("{:?}", list);

        let str_list = Cons("just",
                            Box::new(Cons("a",
                                          Box::new(Cons("test",
                                                        Box::new(Nil))))));
        println!("{:?}", str_list);

        let anno_list = Ele(123, "first",
                            Box::new(Ele(1265464, "second",
                                         Box::new(Ele(89898, "third",
                                                      Box::new(Non))))));
        println!("{:?}\n", anno_list);

    }
    {
        drop_usage();
        println!();
        force_drop();
    }

    {
        use std::rc::Rc;
        use my_box::RcList::{Cons, Nil};

        let a = Rc::new(Cons(213,
                     Rc::new(Cons(56,
                                  Rc::new(Nil)))));
        let b = Cons(11, Rc::clone(&a));
        let c = Cons(22, Rc::clone(&a));
        println!("\nRc List:");
        println!("{:?}  {:?}", b, c);
        println!("Rc not remove: {:?}", a);
        println!("Strong count: {}\n", Rc::strong_count(&a));
    }

    {
        use my_box::RcRefList::{Cons, Nil};
        use std::rc::Rc;
        use std::cell::RefCell;

        let value = Rc::new(RefCell::new(vec![]));
        let arc = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let a = Cons(Rc::new(RefCell::new(vec![1])), Rc::clone(&arc));
        let b = Cons(Rc::new(RefCell::new(vec![2])), Rc::clone(&arc));

        (*value.borrow_mut()).push(1000);

        println!("arc: {:?}", arc);
        println!("a: {:?}", a);
        println!("b: {:?}", b);
    }

    {
        use my_box::RefRcList::{Cons, Nil};
        use std::rc::Rc;
        use std::cell::RefCell;

        println!("\nA demo of references cycles");
        let a = Rc::new(Cons(127, RefCell::new(Rc::new(Nil))));
        println!("a init strong count:{}", Rc::strong_count(&a));

        let b = Rc::new(Cons(83, RefCell::new(Rc::clone(&a))));
        println!("After create b, a:{}, b:{}", Rc::strong_count(&a),
                 Rc::strong_count(&b));
        println!("b next item: {:?}", b.tail());

        // Edit a's tail as b
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("After change a tail:");
        println!("a:{}, b:{}", Rc::strong_count(&a), Rc::strong_count(&b));

        // cause references cycles
        // println!("a next: {:?}", a.tail());
    }
}
