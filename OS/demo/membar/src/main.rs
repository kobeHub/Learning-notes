#![feature(llvm_asm)]
#![allow(non_snake_case)]
use std::sync::*;
use std::sync::atomic::{Ordering, fence};
use std::thread;
extern crate rand;

static mut X: u8 = 0;
static mut Y: u8 = 0;
static mut R1: u8 = 0;
static mut R2: u8 = 0;

fn thread1(start: Arc<Barrier>, end: Arc<Barrier>) {
    thread::spawn(move|| {
        loop {
            start.wait();
            while rand::random::<u32>() % 8 != 0 {}
            unsafe {
                X = 1;
                // swap these two lines for a `mfence` and not just compiler reordering
                // llvm_asm!("mfence" ::: "memory" : "volatile");
                // llvm_asm!("" ::: "memory" : "volatile");
                fence(Ordering::SeqCst);
                R1 = Y;
            }
            end.wait();
        }
    });
}

fn thread2(start: Arc<Barrier>, end: Arc<Barrier>) {
    thread::spawn(move|| {
        loop {
            start.wait();
            while rand::random::<u32>() % 8 != 0 {}
            unsafe {
                Y = 1;
                // swap these two lines for a `mfence` and not just compiler reordering
                // llvm_asm!("mfence" ::: "memory" : "volatile");
                // llvm_asm!("" ::: "memory" : "volatile");
                fence(Ordering::SeqCst);
                R2 = X;
            }
            end.wait();
        }
    });
}

fn main() {
    let start = Arc::new(Barrier::new(3));
    let end = Arc::new(Barrier::new(3));

    thread1(start.clone(), end.clone());
    thread2(start.clone(), end.clone());

    let mut detected = 0;
    let mut iterations = 1;
    loop {
        unsafe {
            X = 0;
            Y = 0;
        }
        start.wait();
        end.wait();
        if unsafe {R1 == 0 && R2 == 0} {
            detected += 1;
            println!("{} reorders detected after {} iterations", detected, iterations);
            unsafe {

            println!("x {}, y {}, r1 {}, r2 {}", X, Y, R1, R2);
            }
            break;
        }
        iterations += 1;
    }
}
