//! # mock
//!
//! A library to track the number of API call
//! and react to specific number. It would send
//! `Message`, `Email` etc. Use `Message` trait
//! to send.

/// Send a message
pub trait Messenger {
    fn send(&self, msg: &str);
}

/// Track the number of API call and react.
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent = self.value as f64 / self.max as f64;
        if percent >= 1.0 {
            self.messenger.send("Error: value is too large");
        } else if percent >= 0.9 {
            self.messenger.send("Warning: you are using up your 90% API call");
        } else if percent >= 0.75 {
            self.messenger.send("Warning: you are using up your 75% API call");
        }
    }
}

/// 测试替身（test double) 使用另外一个类型测试该类型
/// mock 对象是特定类型的测试替身，但是需要对于不可变
/// 借用的值进行改变时，可以用到 std::cell::RefCell
/// 使用其内部可变性

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        message: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                message: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_mseeage() {
        let mm = MockMessenger::new();
        let mut lt = LimitTracker::new(&mm, 100);

        lt.set_value(79);

        assert_eq!(mm.message.borrow().len(), 1);
    }
}
