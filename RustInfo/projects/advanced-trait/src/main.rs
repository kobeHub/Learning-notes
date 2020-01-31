use std::ops::Add;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Millimeters(i32);
struct Meters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

mod spefic {
    pub trait Pilot {
        fn fly(&self);
        fn anno();
    }

    pub trait Wizard {
        fn fly(&self);
        fn anno();
    }

    pub struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is the captian speaking");
        }

        fn anno() {
            println!("Anno from Pilot");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }

        fn anno() {
            println!("Anno from Wizard");
        }
    }

    /*
    impl Human {

        fn fly(&self) {
            println!("Human can't fly");
        }
    }*/
}

mod super_trait {
    use std::fmt;

    pub trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
}

use super_trait::OutlinePrint;

impl OutlinePrint for Point {}

fn main() {
    let p1 = Point {
        x: 12,
        y: 65,
    };
    let p2 = Point {
        x: 321,
        y: -13,
    };

    let p = p1 + p2;
    println!("Ops Add overload: {:?}", p);
    p.outline_print();

    let m = Meters(21);
    let mm = Millimeters(1231);
    println!("Millimeters + Meters: {:?}", mm + m);

    use crate::spefic::{Human, Pilot, Wizard};
    let h = Human;
    Pilot::fly(&h);
    Wizard::fly(&h);
    <Human as Pilot>::anno();
    <Human as Wizard>::anno();
    // h.fly();
    println!();

    use advanced_trait::{Status, returns_closure};
    let list_status: Vec<Status> = (0i32..20)
        .map(Status::Value)
        .collect();
    println!("{:?}", list_status);
    let add_one = returns_closure();
    println!("1 + 1: {}", add_one(1));
}
