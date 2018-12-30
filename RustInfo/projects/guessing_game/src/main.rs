use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    // Gen a random i32 range(1, 100)
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("Just guess the number between 1 and 100!");


    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("[-]Fail to read line!");


        // Transfer guess from string to i32, rust allow a new
        // variable to be shadow variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your answer: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You've got it! Answer:{}", secret_num);
                break;
            }
        }
    }
}
