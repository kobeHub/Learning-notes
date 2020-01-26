use workout::generate;
// use workout::iterator_test::{self, Pair};


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_num = 7;

    generate::generate_work_out(simulated_user_specified_value, simulated_random_num);
    //process_long::
    move_to_clouster();
    generate::use_mut_fn();

    //let test = (0..10); std::ops::Range
    //let () = test;
}

fn move_to_clouster() {
    let x = 123;
    let equal_to_x = |z| z == x;
    println!("{} equal to {}: {}", 125, x, equal_to_x(125));

    let equal = move |z| z == x;
    println!("{}", equal(123));
}
