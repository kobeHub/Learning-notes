#![allow(dead_code)]
use std::mem;
use std::any;
use std::marker::PhantomData;
use zerocopy::Unaligned;


#[derive(Debug)]
struct Student {
    age: u8,
    sex: u32,
    a: u16,
}

#[derive(Debug)]
struct A(u8, u16, f64);


#[derive(Debug)]
struct B(i32, u64, u16);

struct Foo<T, U> {
    cnt: u64,
    a: T,
    b: U,
}

trait ClickCallBack: std::fmt::Debug {
    fn on_click(&self, x: i64, y: i64);
}

#[derive(Debug)]
struct Void;

#[repr(u8)]
enum Enum1 {
    V1,
    V2(u8),
    V3(u16)
}

// #[repr(u16)]
enum Enums {
    V1, V2, V3
}

fn print_size<T>() { 
    println!("type {} size {}, align: {}", any::type_name::<T>(), mem::size_of::<T>(), mem::align_of::<T>());
}

// `align` and `packed`
#[repr(C)]
struct Basic {
    a: u8,
    b: u32, 
    c: u16,
}


#[repr(C, align(8))]
struct Basic1 {
    a: u8,
    b: u32, 
    c: u16,
}

#[repr(C, align(16))]
struct Basic2 {
    a: u8,
    b: u32, 
    c: u16,
}

#[repr(C, packed(2))]
struct Basic3 {
    a: u8,
    b: u32, 
    c: u16,
}

// #[repr(align(16), packed(2))]
// struct AA {
//     a: u16,
// }
// ERROR

#[repr(transparent)]
#[derive(Debug)]
struct Trans1 {
    v: u8,
    _t: Void,
}

#[repr(C)]
struct Trans2 {
    v: u8,
}


fn main() {
    print_size::<u64>();
    print_size::<i64>();
    print_size::<f32>();
    print_size::<f64>();
    print_size::<i128>();
    print_size::<u128>();
    print_size::<isize>();
    print_size::<usize>();
    print_size::<Student>();
    print_size::<A>();
    print_size::<B>();
    print_size::<[f64;32]>();
    print_size::<char>();
    print_size::<Foo<u16, u32>>();
    print_size::<Foo<u32, u16>>();
    print_size::<Foo<u16, u16>>();
    print_size::<Void>();
    print_size::<Box<dyn std::fmt::Debug>>();
    print_size::<Box<dyn ClickCallBack>>();
    // print_size::<dyn ClickCallBack>(); DST cannot be known
    print_size::<Enum1>();
    print_size::<Enums>();
    print_size::<Basic>();
    print_size::<Basic1>();
    print_size::<Basic2>();
    print_size::<Basic3>();
    print_size::<Trans1>();
    print_size::<Trans2>();

    let b3 = Basic2{a: 1, b: 2, c:3};
    println!("Basic2 fileds address, a: 0x{:X}, b: 0x{:X}, c: 0x{:X}", 
             &b3.a as *const u8 as usize, &b3.b as *const u32 as usize, &b3.c as *const u16 as usize);
    
    let trans1 = Trans1 {v: 12, _t: Void{}};
    println!("Transparent representation {:?}, add 1 {:?}", trans1, trans1.v + 1);
}
