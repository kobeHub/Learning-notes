/// Common `Draw` trait to draw GUI
pub trait Draw {
    fn draw(&self);
}

/// Object store `Draw`, to get sharing behaviors
/// use `dyn` to store variable types.
/// 使用trait bounds 的泛型定义只可以存储一种类型
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

pub mod gui {
    use super::*;

    pub struct Button {
        pub width: i32,
        pub height: i32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Draw button: {}", self.label);
        }
    }

    pub struct SelectBox {
        pub width: i32,
        pub height: i32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Draw selectbox: {:?}", self.options);
        }
    }
}
