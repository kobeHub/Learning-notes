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
use crate::string::slice;

mod structs;
use crate::structs::create_struct;
use crate::structs::rectangle;

mod enums;
use crate::enums::ip_addr;
use crate::enums::message;
use crate::enums::coin;
use crate::enums::option;


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
   let s: String = "Hello".into();
   let (mut s1, len) = str_string::calculate_length(s);
   println!("String:{}, length:{}", s1, len);

   str_string::append(&mut s1);
   println!("After append:{}", s1);
   str_string::mut_refer();
   println!("Find first word:{}",
            slice::first_world(&"Find fiest word!"));

   println!();
   let user = String::from("Kobe Bryant");
   let email = String::from("inno@gmail.com");
   create_struct::create(email, user);

   println!();
   create_struct::tuple_struct();

   println!();
   let rect = rectangle::Rectangle(12, 13);
   println!("{:?} use area function: {}", rect, rectangle::Area(&rect));
   println!("use rectangle methods:{}", rect.area());
   let square = rectangle::Rectangle::Square(11);
   println!("The square is:{:?} area: {}", square, square.area());
   println!("first rect can hold the second? {}", rect.can_hold(&square));

   let mut rect1 = rectangle::Rectangle(10, 5);
   println!("Now increase rect1 {:?} by (12, 10)", rect1);
   rect1.Increase(12, 10);
   println!("{:#?}", rect1);

   println!();
   let host: String = "127.0.0.1".into();
   ip_addr::ip_struct(host);

   println!("The ip address use enums:");
   let local4 = ip_addr::IpAddr::V4(127, 0, 0, 1);
   let local6 = ip_addr::IpAddr::V6(String::from("::1"));
   println!("{:#?}, {:#?}", local4, local6);
   println!("\nnow send a Quit message\n...");
   let quit = message::Message::Quit(local4);
   println!("{:?}", quit);
   quit.call();

   println!();
   let penny = coin::Coin::Penny;
   println!("Value of the coin:{}, {:?}", coin::value_in_coin(&penny), penny);
   let quater = coin::Coin::Quarter(coin::UsState::Alabama);
   println!("Value of quater:{}", coin::value_in_coin(&quater));
   coin::quarter(&quater);

   println!("\nThe option usage:");
   let mut five = Some(5);
   let mut no = None;
   option::plus_one(&mut five);
   option::plus_one(&mut no);
   println!("{:?}\t{:?}", five, no);
   let six = Some(6);
   let none = None;
   println!("{:?}, {:?}", option::Plus_one(six), option::Plus_one(none));

   let (uint1, uint2) = (1u8, 100u8);
   option::some_u8_value(&uint1);
   option::some_u8_value(&uint2);


}
