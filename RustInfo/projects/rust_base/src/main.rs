// The basic common programing feature of Rust
//use std::intrinsics::type_name;


////print a variable type use nightly-build api
//#![feature(core_intrinsice)]
//
//fn get_type_of<T>(_: &T) {
//    println!("{}", unsafe { type_name::<T>() });
//}

mod string;

mod structs;
use crate::structs::{create_struct, rectangle};


mod enums;
use crate::enums::{ip_addr, message, coin, option};
//use crate::enums::message;
//use crate::enums::coin;
//use crate::enums::option;

mod collect;

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
    immutable_test();
    rev_out();
    format_print();

    {
        println!("String usage:");
        use crate::string::{str_string, slice};
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
    }

    println!("\n\n");

    {
        println!("Struct usage:");
        let user = String::from("Kobe Bryant");
        let email = String::from("inno@gmail.com");
        create_struct::create(email, user);

        println!();
        create_struct::tuple_struct();

        println!();
        let rect = rectangle::rectangle(12, 13);
        println!("{:?} use area function: {}", rect, rectangle::area(&rect));
        println!("use rectangle methods:{}", rect.area());
        let square = rectangle::Rectangle::square(11);
        println!("The square is:{:?} area: {}", square, square.area());
        println!("first rect can hold the second? {}", rect.can_hold(&square));

        let mut rect1 = rectangle::rectangle(10, 5);
        println!("Now increase rect1 {:?} by (12, 10)", rect1);
        rect1.increase(12, 10);
        println!("{:#?}", rect1);
    }

    println!("\n\n");

    {
        println!("Enumerations and match usage:");
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
        println!("{:?}, {:?}", option::plus_one_(six), option::plus_one_(none));

        let (uint1, uint2) = (1u8, 100u8);
        option::some_u8_value(&uint1);
        option::some_u8_value(&uint2);

    }

    println!("\n\n");

    {
        println!("\nThe vector part:");
       //mod collect;
        use crate::collect::{string, vector, hashmap, solution};
        vector::test_vec();
        let v1 = vec![12, 23, 55];
        let mut v2 = vec![12, 55, 7];
        vector::iter_vec(&v1);
        vector::add_one(&mut v2);
        println!("{:?}", v2);
        println!("String from collect:");

        string::represent();
        string::build_string();
        string::length_test();
        string::three_format();

        println!("HashMap usage:");
        hashmap::create();
        hashmap::update();
        let text = "It's just a test and test again for usage";
        hashmap::count_words(text);


        let mut data = vec![12, 55, 67, 1, 0, 88, 9];
        println!("Get the feature of the array: {:?}", data);
        let (mean, middle, mode) = solution::number_feature(&mut data);
        println!("The mean:{}, middle number:{}, mode number:{}",
                 mean, middle, mode);
        let word1 = "apple";
        let word2 = "translate";
        let re1 = solution::big_latin(&word1);
        let re2 = solution::big_latin(&word2);
        println!("Translate {}, {} to big latin: {}, {}",
                 word1, word2, re1, re2);
        use std::collections::HashMap;

        let mut records = HashMap::new();
        let mut vec = vec![String::from("Leborn"), String::from("Kobe")];
        records.insert(String::from("base"), &mut vec);
        println!("Before insert: {:?}", records);
        let emp1 = "Jack Milocvi".to_string();
        let part = "sail".to_string();
        solution::employee_simu(&emp1, &part, &mut records);
        println!("After insert:{:?}", records);
    }
}
