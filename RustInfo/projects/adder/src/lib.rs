#[cfg(test)]
mod tests {
    use super::shapes::Rectangle;
    use super::msg;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn return_result() -> Result<(), String> {
        if 2 + 2 == 3 {
            Ok(())
        } else {
            Err(String::from("two add two does not equal three"))
        }
    }

    #[test]
    fn rectangle_small_can_not_hold_large() {
        //use crate::shapes::Rectangle;
        let r1 = Rectangle {
            width: 34.,
            length: 120.1, 
        };
        let r2 = Rectangle{length: 12.5, width: 7.};
        assert!(!r2.can_hold(&r1));
    }

    #[test]
    fn rectangle_large_can_hold_small() {
        let r1 = Rectangle{length: 12.9, width: 22.};
        let r2 = Rectangle{length: 22., width:23.};
        assert!(r2.can_hold(&r1));
    }

    #[test]
    fn greeting_not_done() {
        let result = msg::greeting("Cargo");
        assert!(
            result.contains("Cargoo"),
            "Greeting did not contain the name value:{}", result    
        );
    }

    #[test]
    #[should_panic(expected = "The incorrect value of rectangle: 12.1, -5")]
    fn width_negative() {
        Rectangle::new(12.1, -5.);
    }

    #[test]
    #[should_panic(expected = "The incorrect value of rectangle: 0, 89")]
    fn length_negative() {
        Rectangle::new(0., 89.);
    }
}

pub mod shapes {
    #[derive(Debug)]
    pub struct Rectangle {
        pub length: f64,
        pub width: f64,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    // shold panic usage cases
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
}

pub mod msg {
    pub fn greeting(name: &str) -> String {
        format!("Hello, {}", name)
    }
}
