// Solution for the task after Collections
use std::collections::HashMap;

// Extract the mean, middle, mode number of the input vector
pub fn number_feature(data: &mut Vec<i32>) -> (f64, i32, i32) {
    let mut sum = 0;
    let mut times = HashMap::new();
    for i in data {
        sum += *i;
        let time = times.entry(i).or_insert(0);
        *time +=  1;
    }
    let mean = (sum as f64) / (data.len() as f64);
    data.sort();
    let middle = data[data.len() / 2];
    let mut max = 0;
    let mut mode = 0;
    for (k, v) in times {
        if v > max {
            max = v;
            mode = *k;
        }
    }
    (mean, middle, mode)
}

// Trans a word to big latin
pub fn big_latin(word: &str) -> String {
    const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    //let len = word.len();
    let word = word.to_lowercase();
    let first = word.chars().nth(0);
    let word = &word[..];

    for i in VOWEL.iter() {
        if first == Some(*i) {
            return String::from(&word[1..]) + &word[0..0] + "ay";
        }
    }
    return format!("{}-hay", word);
}


pub fn employee_simu(employee: &String, depart: &String, records: &mut HashMap<String, &mut Vec<String>>) {
    if let Some(v) = records.get(depart) {
        v.push(*employee);
    }
}
