// Solution for the task after Collections
use std::collections::HashMap;

// Extract the mean, middle, mode number of the input vector
pub fn number_feature(data: &mut Vec<i32>) -> (f64, i32, i32) {
    let mut sum = 0;
    let mut times: HashMap<i32, i32> = HashMap::new();
    let data1 = data.clone();
    for i in data1 {
        sum += i;
        let time = times.entry(i).or_default();
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
            mode = k;
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
    // let word = &word[..];

    for i in VOWEL.iter() {
        if first == Some(*i) {
            return format!("{}-hay", word);
        }
    }
    return String::from(&word[1..]) + "-" + &word[0..1] + "ay";
}

/*
pub fn employee_simu(employee: &str, depart: &str, records: &mut HashMap<String, &mut Vec<&str> >) {
    if let Some(v) = records.get(depart) {
        v.push(employee);
    }
}*/

#[derive(Debug)]
pub struct EmployeeTable {
   table: HashMap<String, Vec<String>>,
}

impl EmployeeTable {
    pub fn new() -> EmployeeTable {
        EmployeeTable{
            table: HashMap::new(),
        }
    }

    pub fn insert_into(&mut self, deploy: &str, employee: &str) {
        self.table.entry(deploy.to_string())
            .and_modify(|e| {
                let tmp = e.clone();
                for i in tmp {
                    if i == employee {
                        return;
                    }
                }
                (*e).push(employee.to_string());
            })
            .or_insert(vec![employee.to_string()]);
    }

    pub fn get_deploy(&self, deploy: &str) -> Option<&Vec<String>> {
        self.table.get(deploy)
    }

    pub fn summary(&self) -> &HashMap<String, Vec<String>> {
        &self.table
    }
}
