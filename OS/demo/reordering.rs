use {
    std::sync::atomic::{AtomicBool, AtomicU32, Ordering},
    std::sync::{Arc, Barrier},
    std::thread,
};

/// 测试指令重排，一个标志位Flag, 一个配置值u32, 没有关联
/// 可能进行指令重排
#[derive(Debug)]
struct Config {
    d: AtomicU32,
}
/// t1         t2
///            get(co)
/// write(co)
/// write(flag)
///            get(flag)
///
///
/// 线程可能的执行状态

fn reorder() {
    let flag = Arc::new(AtomicBool::new(false));
    let f1 = flag.clone();
    let f2 = flag.clone();
    let co = Arc::new(AtomicU32::new(0));
    let c1 = co.clone();
    let c2 = co.clone();
    //let start = &Arc::new(Barrier::new(1));
    let end = Arc::new(Barrier::new(2));
    //let s1 = start.clone();
    //let s2 = start.clone();
    let e1 = end.clone();
    let e2 = end.clone();

    let mut runs = vec![];
    runs.push(thread::spawn(move || {
        //s1.wait();
        c1.store(1787891f64.sin() as u32 * 128 + 12, Ordering::Release);
        f1.store(true, Ordering::Release);
        e1.wait();
    }));
    runs.push(thread::spawn(move || {
        e2.wait();
        let config = c2.load(Ordering::Acquire);
        let flag = f2.load(Ordering::Acquire);
        if config == 0 && flag {
            println!("reordering.. {:?} {:?}", co, flag);
        }
        //e2.wait();
    }));
    for t in runs {
        let _ = t.join().expect("join failed");
    }
}

fn main() {
    loop {
        reorder();
    }
}
