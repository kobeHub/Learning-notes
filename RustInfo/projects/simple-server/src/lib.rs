use std::fs;
use std::time::Duration;
use std::thread::{self};
use std::sync::{mpsc, Mutex, Arc};
use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    /***************** version 0.1*********************************
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("src/hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("src/404.html").unwrap();

        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}*/

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "src/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK \r\n\r\n", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //println!("Requst: {}", String::from_utf8_lossy(&buffer[..]));
}

/// The job abstraction: the progress to be executed
/// in a thread, it is a closure. The type should be same
/// as `thread::spawn`
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

/// The `Message` send in the channel to implement
/// graceful shutdown
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Build a thread pool
    /// n: thr num of threads
    ///
    /// # Panic
    ///
    /// `new` panic when n is 0
    /// 使用`Send` trait 进行任务分发, 每一个 `Worker` 具有一份
    /// 接收器的原子引用计数拷贝，接受所分配的函数
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0);

        let mut workers = Vec::with_capacity(n);

         let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job: Job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 为了避免死锁，需要使用两个循环，在该循环中
        // 发送Terminate信号
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Close worker {}", worker.id);

            // 无法通过引用获取所有权是，可以定义该字段为Option，使用
            // `take` 方法获取所有权
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// The actually worker of the threads
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let msg = receiver
                    .lock()
                    .expect("ThreadPool Fatal: the Mutex has been poisoned!")
                    .recv()
                    .expect("ThreadPool Fatal: the receiver recv error");
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing..", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} receives terminate signal", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
