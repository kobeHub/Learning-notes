use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

fn spawn_usage() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("No:{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("NO:{} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn use_move_closure() {
    let v = vec![2131, 321, 90];
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn channel_send() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "just",
            "a",
            "test",
            "rust...",
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });
    // let va = rx.recv().unwrap();
    //  println!("{}", va);

    for rec in rx {
        println!("Got: {}", rec);
    }
}

fn multi_senders() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            "Kobe",
            "RIP",
            "are",
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "Bryant",
            "you",
            "my",
            "hero",
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for rec in rx {
        println!("Got:{}", rec);
    }
}

use std::sync::Arc;

// Use 10 threads to test Mutex
fn simple_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn simple_mutex_v2() {
    const N: usize = 10;

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();
    for i in 0..N {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
            println!("Thread:{}, num:{}", i, num);
            if *num == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
}

fn dead_lock() {
    let data1 = Arc::new(Mutex::new(100));
    let data2 = Arc::new(Mutex::new(109));

    let (tx, rx) = mpsc::channel();

    let (d1, d2, tx1) = (Arc::clone(&data1), Arc::clone(&data2), tx.clone());
        thread::spawn(move || {
            let num1 = d1.lock().unwrap();
            thread::sleep(Duration::from_millis(100));
            let num2 = d2.lock().unwrap();
            println!("{}, {}", num1, num2);
            tx1.send(()).unwrap();
     });

    let (d1, d2, tx2) = (Arc::clone(&data1), Arc::clone(&data2), tx.clone());
    thread::spawn(move || {
            let num1 = d2.lock().unwrap();
            thread::sleep(Duration::from_millis(100));
            let num2 = d1.lock().unwrap();
            println!("{} {}", num1, num2);
            tx2.send(()).unwrap();
    });

    rx.recv().unwrap();
}

fn main() {
    spawn_usage();

    println!();
    use_move_closure();
    println!();
    channel_send();

    println!();
    multi_senders();
    println!();
    simple_mutex();
    println!();
    simple_mutex_v2();
    println!();
    dead_lock();
}
