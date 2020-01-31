/// 高级函数与闭包 Rust 支持函数式编程
/// 所以函数可以作为第一成员，作为函数的参数或返回值
/// fn 关键字定义的类型是函数指针
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice (f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

#[derive(Debug)]
pub enum Status {
    Value(i32),
    Stop,
}

pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_use_fn_pointer_as_args() {
        let answer = do_twice(add_one, 2);
        assert_eq!(6, answer);
    }

    #[test]
    fn it_work_with_closure() {
        let list = vec![1, 2, 3];
        let list_string: Vec<String> = list
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(vec![String::from("1"), String::from("2"), String::from("3")],
        list_string);
    }
}
