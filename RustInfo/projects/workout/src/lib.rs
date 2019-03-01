pub mod generate {
    use crate::simulated:: Cacher;
    use std::{thread, time::Duration};

    /* Generate the workout plan via an expensive calculation */
    pub fn generate_work_out(intensity: u32, random_num: u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating...");
            thread::sleep(Duration::from_secs(2));
            num
        });
        
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                expensive_result.value(intensity)
                //simulated::simulated_expensive_calculation(intensity)
            );
            println!(
                "Next, do {} situps!",
                expensive_result.value(intensity-1)
                //simulated::simulated_expensive_calculation(intensity)
            );
        } else {
            if random_num == 3 {
                println!("Take a break today!");
            } else {
                println!(
                    "Today, run {} minutes",
                    expensive_result.value(intensity-1)
                    //simulated::simulated_expensive_calculation(intensity)
                );
            }
        }
    }
}

mod simulated {
    //use std::time::Duration;
    //use std::thread;
    /* Try to simulate an expensive calculation */
    //#[warn(dead_code)]
    //pub fn simulated_expensive_calculation(num: u32) -> u32 {
    //    println!("calculating...");
    //    thread::sleep(Duration::from_secs(2));
    //    num
    //}

    use std::collections::HashMap;
    /* The Cacher to reduce the call of the expensive calculation */
    pub struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        /*Default value of the func call is None */
        pub fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        pub fn value(&mut self, arg: u32) -> u32 {            
            match self.value.get(&arg) {
                None => {
                    let v = (self.calculation)(arg);
                    self.value.insert(arg, v);
                    v
                },
                Some(&v) => v,
            }
        }
    }
}
/*
pub mod process_long {
    use crate::simulated::Cacher;
    use std::{thread, time::Duration};

    pub fn str_expensive() {
        let str1 = "Just a str";
        let str2 = "Wait anyway";
        let str3 = "another one";

        let expensive_result = Cacher::new(|slice| {
            println!("Process expensive: {}", slice);
            thread::sleep(Duration::from_secs(2));
            slice.len()
        });
        expensive_result.value(str1);
        expensive_result.value(str1);
        expensive_result.value(str2);
        expensive_result.value(str3);
        expensive_result.value(str2);

    }
}*/

#[cfg(test)]
mod test {
    use super::iterator_test::{self, Shoe, Pair, Counter};
    #[test]
    fn file_shoes_in_my_size() {
        let shoes = vec![
            Shoe {size: 12, style: String::from("sneaker")},
            Shoe {size: 11, style: String::from("boot")},
            Shoe {size: 11, style: String::from("sandl")},
        ];

        let shoes_in_my_size = iterator_test::shoes_in_my_size(shoes, 11);
        assert_eq!(
            shoes_in_my_size,
            vec![
                Shoe {size: 11, style: String::from("boot")},
                Shoe {size: 11, style: String::from("sandl")},
            ]
        );
    }

    #[test]
    fn pair_lifetime() {
        let mut p1 = Pair::new(1288889, "Leborn James", 0);
        assert_eq!(p1.name(), "Leborn James");
        assert_eq!(p1.id(), 1288889);
    
        assert_eq!(p1.next(), Some((1288889, "Leborn James")));    
        assert_eq!(p1.next(), Some((1288889, "Leborn James")));    
        assert_eq!(p1.next(), Some((1288889, "Leborn James")));    
        assert_eq!(p1.next(), Some((1288889, "Leborn James")));    
        assert_eq!(p1.next(), Some((1288889, "Leborn James")));    
        assert_eq!(p1.next(), None);    
        //assert_eq!(p1.next(), Some(Pair::new(1288889, "Leborn James", 2)));    
        //assert_eq!(p1.next(), Some(Pair::new(1288889, "Leborn James", 3)));    
        //assert_eq!(p1.next(), Some(Pair::new(1288889, "Leborn James", 4)));    
        //assert_eq!(p1.next(), Some(Pair::new(1288889, "Leborn James", 5)));    
        //assert_eq!(p1.next(), None);    
    }

    #[test]
    fn use_other_of_iterator() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

pub mod iterator_test {
    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String,
    }

    // filter my size shoes in the given Vec
    pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)  // get varaible from the environment
            .collect()
    }

    /* Define the personal type to impl the Iterator trait */
    #[derive(PartialEq, Debug)]
    pub struct Pair<'a> {
        id: u32,
        name: &'a str,
        count: u8,
    }

    /* The basic function for the Pair */
    impl<'a> Pair<'a> {
        pub fn new(id: u32, name: &'a str, count: u8) -> Pair {
            Pair {id, name, count}
        }

        pub fn name(&self) -> &str {
            self.name
        }

        pub fn id(&self) -> u32 {
            self.id
        }
    }

    impl<'a> Iterator for Pair<'a> {
        type Item = (u32, &'a str);    // Define the `next()` method return type

        // Use the method of the `Iterator` trait to get the 
        // element tuple for six times
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some((self.id, self.name))
            } else {
                None
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Counter {
        count: u32
    }

    impl Counter {
        pub fn new() -> Counter {
            Counter {count: 0}
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
}
