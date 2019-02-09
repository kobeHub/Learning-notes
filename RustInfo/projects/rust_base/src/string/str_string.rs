/* Test for str and string*/
//pub fn str_string() {
//    let a = "It's a str";
//    let b = String::from("It's a string");
//    //let () = a;
//    //let () = b;
//    println!("{}, {}", a, b);
//}

pub fn string_test() {
    println!("Just a simple usage of String");
    println!("3 methods to initialize a String:");
    println!("str.to_string(), String.from(str), str.into()");
    let origin = "Hello, it's the origin str";
    let mut str1 = String::from(origin);
    let str2: String = origin.into();
    let str3 = origin.to_string();

    str1.push_str(", and the mutable str is me!");
    println!("{}, {}, {}", str1, str2, str3);

    let heart = vec![240, 159, 146, 150];
    println!("\nfrom a bytes vec:{:#?}", heart);
    let heart = String::from_utf8(heart).unwrap();
    println!("{}", heart);
}

// return the ownership of string by return string
pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// mutable reference to modify the value
pub fn append(some_string: &mut String) {
    some_string.push_str(", Rustean");
}

pub fn mut_refer() {
    let s = String::from("the mutable string");
    let s1 = &s;
    let s2 = &s;
    println!("{}{}", s1, s2);
}

pub fn copy_test() {
    let x = 5;
    let y = x;
    let str1 = String::from("It's just a string");
    let str2 = str1.clone();
    println!("{}, {}, {}, {}", x, y, str1, str2);
}
