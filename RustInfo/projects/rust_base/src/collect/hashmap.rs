/*Basic usage of the hashmap*/
use std::collections::HashMap;

pub fn create() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 90);

    let teams = vec![String::from("Leborn James"), String::from("Steven Curry")];
    let ini_score= vec![10, 5];
    let team_scores: HashMap<_, _> = teams.iter().zip(ini_score.iter()).collect();
    println!("The two hashmap:");
    println!("{:?}, {:?}", scores, team_scores);
    for (k, v) in &scores {
        print!("{}; {}; ", k, v);
    }
    println!();
    for (k, v) in &team_scores {
        print!("{}: {}; ", k, v);
    }
    println!();

    println!("Now try to get the score of the LBJ team:");
    if let Some(v) = team_scores.get(&String::from("Leborn James")) {
        println!("Score is: {}", v);
    } else {
        println!("Error key!");
    }
    match team_scores.get(&String::from("Hardon")) {
        Some(v) => println!("Hardon's socre: {}", v),
        None => println!("There is no Hardon key")
    }
}

pub fn update() {
    let mut scores: HashMap<String, i64> = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 100);
    scores.insert(String::from("red"), 50);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow"))
        .and_modify(|e| { *e += 1;})
        .or_insert(90);
    scores.entry(String::from("Pink")).or_insert(88);
    scores.insert(String::from("Red"), 1999);
    scores.entry(String::from("Green")).or_default();
    println!("{:?}", scores);
}

pub fn count_words(words: &str) {
    let mut map = HashMap::new();
    let entry = map.entry("test");
    println!("{:#?}", entry);
    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
