/* use genertic parameter to write shorter code */

// Find the largest element of a list
use std::cmp::PartialOrd;

// use trait bounds 替代 impl TRAIN
// 实现了Copy trait 的类型，必须是可以进行简单内存拷贝的类型
// 也就是说必须是数字类型/bool/共享借用指针 或者由其组成的类型
// PartialOrd trait 是指满足偏序关系的元素集合，与全序不同，偏序不保证
// 集合中所有的元素均满足该关系，例如： 可比较是复数集上的偏序关系
// 偏序关系只需要满足：自反、反对称、传递
// 全序关系需要满足：自反、反对称、全序
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // The type which impls PartialOrd and Copy trait can use
    // the function
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Use a lifttime annotions to get longest str
pub fn str_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//function with a lifttime ignore
// 生命周期计算过程： 1. 每一个参数都有自己的生命周期
// 2. 只有一个参数时，输入生命周期，泽被赋予所有输出生命周期
// 3. 有多个参数，且有&self/&mut self 时，self的生命周期被赋予所有的输出生命周期
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}
