use crate::ip_addr;

// enums of message
#[derive(Debug)]
pub enum Message {
    Quit(ip_addr::IpAddr),
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        println!("you are calling methods of Message:{:?}", self);
    }
}
