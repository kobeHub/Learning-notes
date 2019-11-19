// Use match to get option value
pub fn plus_one(x: &mut Option<i32>) {
    match x {
        None => {},
        Some(x) => {
            *x += 1;
        }
    }
}

pub fn plus_one_(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }

}

pub fn some_u8_value(x: &u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    }
}
