/* Functions use generics */
use std::fmt::{Debug, Display};

pub fn total_length<T, U>(t: &[T], u: &[U]) -> usize
    where T: Display + Clone + Debug,
          U: Clone + Debug

{
    println!("The data of parameter:");
    println!("{:?}, {:?}", t, u);

    t.len() + u.len()
}

pub fn longest_with_anno<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display 
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
