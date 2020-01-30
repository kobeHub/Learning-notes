use std::slice;

fn split_as_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

// use C ABI
extern "C" {
    fn abs(input: i32) -> i32;
}

#[link(name = "hello", kind = "static")]
extern "C" {
    pub fn say_hello();
}

static mut COUNTER: u32 = 321;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    let mut num = 1321321;

    let address = &num;
    let rc = address as *const i32;
    let rm = &mut num as *mut i32;

    unsafe {
        println!("Amazing... {}", *rc);
        *rm += 1;
        println!("After add: {}", *rc);
    }

    let data = &mut [321, 89, 32, 90];
    let (first, others) = split_as_mut(data, 1);
    (*first)[0] += 1;
    println!("first: {:?}", first);
    println!("others: {:?}", others);

    println!("\nC extern usage:");
    unsafe {
        println!("Abs of -32732: {}", abs(-32732));
    }
    unsafe {
        say_hello();
    }

    println!("Access and edit static variables is unsafe");
    add_to_count(12);
}
