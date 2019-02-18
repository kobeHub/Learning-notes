/* use genertic parameter to write shorter code */

// Find the largest element of a list
use std::cmp::PartialOrd;
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
