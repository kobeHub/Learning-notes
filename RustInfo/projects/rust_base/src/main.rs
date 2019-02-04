// The basic common programing feature of Rust
//use std::intrinsics::type_name;


////print a variable type use nightly-build api
//#![feature(core_intrinsice)]
//
//fn get_type_of<T>(_: &T) {
//    println!("{}", unsafe { type_name::<T>() });
//}

mod string;
use crate::string::str_string;
// Define the trait of join tuple into string
// Then can be printed
trait JoinTuple {
    fn join_tuple(&self, sep: &str) -> String;
}

macro_rules! tuple_impls {
    () => {};

    ( ($idx:tt => $typ:ident), $( ($nidx:tt => $ntyp:ident),  )*  ) => {
        impl<$typ, $( $ntyp  ),*> JoinTuple for ($typ, $( $ntyp  ),*)
        where
            $typ: ::std::fmt::Display,
            $( $ntyp: ::std::fmt::Display  ),*
        {
            fn join_tuple(&self, sep: &str) -> String {
                let parts: &[&::std::fmt::Display] = &[&self.$idx, $( &self.$nidx  ),*];
                parts.iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
            }
        }

        tuple_impls!($( ($nidx => $ntyp),  )*);
    };
}

tuple_impls!(
    (9 => J),
    (8 => I),
    (7 => H),
    (6 => G),
    (5 => F),
    (4 => E),
    (3 => D),
    (2 => C),
    (1 => B),
    (0 => A),
);

fn immutable_test() {
    let num1: i64 = 123;
    let mut str1 = "Just change it!";
    println!("Before change: {} {}", num1, str1);

    str1 = "Now it changed";
    println!("After change: {}", str1);
    let num1 = "Shadowing variable";
    println!("{}", num1);

    let num2 = 1_235u32;
    let num3 = 0xff_ffff;
    let num4 = 0b0000_1111;
    let num5 = 0o77;
    let byte = b'a';
    println!("print the num with type:\n{} {} {} {} {}",
             num2, num3, num4, num5, byte);


    println!("\nUse the string to print tuple:");
    let tuple = (3.3f32, "Just a test", b'f', 12);
    let str1 = tuple.join_tuple(", ");
    println!("{}", str1);
    println!("The first ele of tuple: {}", tuple.0);
    assert_eq!(tuple.0, 3.3);
    //assert_eq!(tuple.0, 3.3f64);
    //
    println!("now iteral the array:");
    let array = [12.0, 9.0 ,8., 11.];
    print!("[");
    for ele in array.iter() {
        print!("{} ", ele);
    }
    println!("]");

    let str3 = if tuple.3 > 8 {
        "It's greater than 8"
    } else {
        "It's less than 8"
    };
    println!("{}", str3);

    let mut count =  100;
    let a = loop{
        count -= 1;
        if count == 10 {
            break count * 3 + 1;
        }
    };
    println!("{}", a);
}

fn rev_out() {
    for i in (1..4).rev() {
        print!("{}, ", i);
    }
    println!("\nDone!");
}

// Formatted print test
fn format_print() {
    println!("\nFormat print test:");
    let str1 = format!("Just a test with {}", "format!");
    println!("{}", str1);
    println!("{one} is {doing} by {two}",
             one="Tim",
             doing="beating",
             two="Tom");
    println!("{:?}", (12.0, 55));
    println!("{:05}", 31);
    println!("{} of {:#b} people know binary, the other do not", 1, 2);
    println!("Padding num with extra 0 {num:0>width$}", num=12, width=5);
    println!("{:0>7.*} {1}", 3, 1.3216324);
    println!("{:0>wid$.*} {1}", 3, 3.1415, wid=6);
    println!("{:x<4}", 12);
}

fn main() {
    //immutable_test();
    //rev_out();
    //format_print();
   str_string::string_test();
   str_string::copy_test();
}
