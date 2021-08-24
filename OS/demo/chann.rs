use {
    std::thread,
    std::sync::mpsc,
};

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for rx in rx.iter() {
        println!("receive {}", rx);
    }
}
