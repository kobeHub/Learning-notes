use {
    std::cell::UnsafeCell,
    std::env,
    std::thread,
    std::sync::{Arc, Barrier},
    std::sync::atomic::{AtomicUsize, Ordering},
};

struct AtomicPair {
    atom: AtomicUsize,
    norm: UnsafeCell<usize>,
}

unsafe impl Sync for AtomicPair {}

impl AtomicPair {
    fn new(v: usize) -> AtomicPair {
        AtomicPair {
            atom: AtomicUsize::new(v),
            norm: UnsafeCell::new(v),
        }
    }

    fn get(&self) -> (usize, usize) {
        // test `Acquire` order, all load ops after `Acquire` wont's 
        // reorder to the front of `Acquire`
        let atom = self.atom.load(Ordering::Relaxed);
        
        let norm = unsafe { *self.norm.get() };
        (atom, norm)
    }

    fn set(&self, v: usize) {
        // test `Release` order, all `store` ops before `Release` wont's
        // reorder to the back of `Release`
        unsafe { *self.norm.get() = v };

        self.atom.store(v, Ordering::Relaxed);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let n_thread: usize = (&args[1]).parse().unwrap();
    let n_iter: usize = (&args[2]).parse().unwrap();
    println!("Use {} threads, set value iter {}", n_thread, n_iter);

    let upair = Arc::new(AtomicPair::new(0));

    let barrier = Arc::new(Barrier::new(n_thread + 1));
    let mut runs = vec![];

    for _ in 0..n_thread {
        let unpair = upair.clone();
        let barrier = barrier.clone();
        runs.push(thread::spawn(move || {
            barrier.wait();

            let mut v = 0;
            while v < n_iter - 1 {
                let (atom, norm) = unpair.get();
                if atom > norm {
                    println!("Reordered! {} > {}", atom, norm);
                }
                v = atom;
            }
        }));
    }

    barrier.wait();

    for v in 1..n_iter {
        upair.set(v);
    }

    for r in runs {
        r.join().unwrap();
    }


}
