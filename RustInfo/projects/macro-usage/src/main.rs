// macro_rules!
#[macro_export]
macro_rules! vect {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push($x);
            )*
                tmp
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::*;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let mine = vect!["just", "a", "vec", "simple", "impl"];
    println!("mine: {:?}", mine);

    let s = Pancakes;
    println!("Create pancakes");
}
