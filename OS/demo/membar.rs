use {
    std::thread,
    std::sync::{
        Arc, Mutex,
    },
};

struct Data {
    a: Mutex<u32>,
    b: Mutex<u32>,
}

fn main() {
    let data = Arc::new(Data {a: Mutex::new(0), b: Mutex::new(0)});
    let dt1 = data.clone();
    let mut dt2 = data.clone();
    let t1 = thread::spawn(move || {
        loop {
            if *dt1.a.lock().unwrap() != 0u32 {
                println!("T1 print b {:?}", data.b.lock().unwrap());
                return;
            }
            println!("T1 print b {:?}",(*data).b.lock().unwrap());
        }
    });
    let t2 = thread::spawn(move || {
        let mut data1 = Arc::get_mut(&mut dt2).unwrap();
        data1.a = Mutex::new(10);
        data1.b = Mutex::new(1);
    });
    t1.join().expect("T1 join failed");
    t2.join().expect("T2 join failed");
}
