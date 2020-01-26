pub mod mybox {
    use std::ops::Deref

    #[derive(Debug)]
    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    pub struct CustomPointer {
        pub data: String,
    }

    impl Drop for CustomPointer {
        fn drop(&mut self) {
            println!("Dropping customer pointer with data: {}", self.data);
        }
    }
}

pub mod list {
    use std::rc::Rc;

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
}
