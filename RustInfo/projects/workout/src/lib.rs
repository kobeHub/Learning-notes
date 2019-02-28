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
