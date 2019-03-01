use std::{env, process};

use minigrep;
use minigrep::Config;

fn main() {
    /*****************version 0.1***************************
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Deal panic and exit the program
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    ********************************************************/

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parseing arguments, {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applcation error: {}", e);
        process::exit(1);
    };

    /*******************code restruct 0.1*********************
    // Instaed of using the expect, use Result to hadle Err
    let content = fs::read_to_string(config.filename)
        .expect("Something went wrong when reading the file");
    println!("The contents: {}", content);
    **********************************************************/
}

