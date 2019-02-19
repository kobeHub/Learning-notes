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
