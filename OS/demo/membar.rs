use {
    std::sync::atomic::{AtomicU32, Ordering},
    std::sync::Arc,
    std::thread,
};

struct Data {
    a: AtomicU32,
    b: AtomicU32,
    c: AtomicU32,
}

fn atomic_usage() {
    let data = Arc::new(Data {
        a: AtomicU32::new(0),
        b: AtomicU32::new(0),
        c: AtomicU32::new(0),
    });
    let dt1 = data.clone();
    let dt2 = data.clone();
    let t1 = thread::spawn(move || {
        dt1.a.store(1, Ordering::Relaxed);
        dt1.b.store(2, Ordering::Relaxed);
        dt1.c
            .store(dt1.a.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    });
    let t2 = thread::spawn(move || {
        let c = dt2.c.load(Ordering::Relaxed);
        let b = dt2.b.load(Ordering::Relaxed);
        let a = dt2.a.load(Ordering::Relaxed);

        println!("a {}, b {}, c {}", a, b, c);
    });
    t1.join().expect("T1 join failed");
    t2.join().expect("T2 join failed");
}

fn main() {
    for _ in 0..10 {
        atomic_usage();
    }
}
