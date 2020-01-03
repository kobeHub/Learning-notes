use std::fs::{self, File};
use std::io::{self, prelude::*, ErrorKind};

fn get_file(filename: &str) -> String {
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Create file failure: {:?}", e),
            }
            other_error => panic!("There is a problem in opening file:{:?}", other_error),
        },
    };
    let mut contents = String::new();    // Pay atten: read_to_string is a method of &mut File
    match f.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(error) => panic!("Read file error:{:?}", error),
    };
    contents
}

// Read file return a Result enum
#[allow(dead_code)]
fn read_from_file(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);

    // unwrap the result
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),    // Result type
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),    // return the contents
        Err(e) => Err(e),  // return the error message
    }
}


// propagating a error case, using a ? operator
#[allow(dead_code)]
fn read_contents(filename: &str) -> Result<String, io::Error> {
    //let mut f = File::open(filename)?;
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;    // 使用`?`操作符的链式法则
    Ok(s)
}

// direct read from file
fn direct_read(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn open_file(filename: &str) -> File {
    File::open(filename).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Tried to creatt file but there is a problem: {:?}", error);
            })
        } else {
            panic!("There is a problem while opening file: {:?}", error);
        }
    }).unwrap()    // use unwrap to get the value or err
}


fn main() {
    //panic!("crash and burn!");
    let f = get_file("JustaTest");
    println!("{}", f);
    let f = open_file("open");
    println!("{:?}", f);
    let contents = direct_read("Cargo.toml").unwrap();
    println!("{}", contents);
}
