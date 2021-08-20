use {
    std::sync::{Arc, Barrier},
    std::thread,
};
// extern crate rand;

static mut X: u32 = 0u32;
static mut Y: u32 = 0u32;
static mut R1: u32 = 0u32;
static mut R2: u32 = 0u32;

fn writer1(start: Arc<Barrier>, end: Arc<Barrier>) {
    thread::spawn(move || {
        loop {
            start.wait();
            // while rand::random::<u32> % 8 != 0 {}
            unsafe {
                X = 1;
                R1 = Y;
            }
            end.wait();
        }
    });
}

fn writer2(start: Arc<Barrier>, end: Arc<Barrier>) {
    thread::spawn(move || {
        loop {
            start.wait();
            // while rand::random::<u32> % 8 != 0 {}
            unsafe {
                Y = 1;
                R2 = X;
            }
            end.wait();
        }
    });
}

fn main() {
    let start = Arc::new(Barrier::new(3));
    let end = Arc::new(Barrier::new(3));

    writer1(start.clone(), end.clone());
    writer2(start.clone(), end.clone());

    let mut detected = 0;
    let mut iterations = 1;
    loop {
        unsafe {
            X = 0;
            Y = 0;
        }
        start.wait();
        end.wait();
        if unsafe { R1 == 0 && R2 == 0 } {
            detected += 1;
            println!(
                "{} reorders detected after {} iterations",
                detected, iterations
            );
        }
        iterations += 1;
    }
}
