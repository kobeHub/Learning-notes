// string usage and test
use std::mem;
pub fn represent() {
    let story = String::from("It's a long story begining with a little ting...");
    let ptr = story.as_ptr();
    let len = story.len();
    let cap = story.capacity();
    println!("Presentaion:{:?}, {}, {}", ptr, len, cap);

    // throw the variable contains the origin data
    mem::forget(story);

    // rebuild the string from 3 parts
    let result = unsafe { String::from_raw_parts(ptr as *mut _, len, cap) };
    println!("{}", result);
}

pub fn build_string() {
    let mut s = String::with_capacity(26);
    println!("{}", s);

    for _ in 0..7 {
        s.push_str("Test");
        println!("{}", s);
    }
}

// utf-8 length
pub fn length_test() {
    let as1 = String::from("Hello");
    let uni1 = String::from("你好啊朋友");
    println!("{}, len:{}", as1, as1.len());
    println!("{}, len:{}", uni1, uni1.len());
}

pub fn three_format() {
    let data = String::from("नमस्ते");
    //let chars = data.chars();
    //println!("{}, {:?}, {:?}", data, bytes, chars);
    for i in data.chars() {
        print!("{} ", i);
    }
    println!();
    for i in data.bytes() {
        print!("{} ", i);
    }
    println!();

}

pub fn string_join() {
    let s1 = String::from("Tatco");
    let s2 = "Tuesday".to_string();
    let s3: String = "LeBorn James".into();
    // s1 onwership move to result
    let result = s1 + "-" + &s2 + "-" + &s3;
    println!("parts: {} {}", s2, s3);
    println!("join: {}", result);
}
