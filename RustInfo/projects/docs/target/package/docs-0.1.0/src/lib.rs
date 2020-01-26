//! # docs
//!
//! `docs` is a Rust docs demo and crate.io usage demo

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod math {
    /// Add one to the spefic `i32`
    ///
    /// # Example
    /// ```
    ///   let n = 125;
    ///   let ans = docs::math::add_one(n);
    ///
    ///   assert_eq!(ans, 126);
    /// ```
    ///
    /// # Common titles:
    /// + Panic
    /// + Errors
    /// + Safety
    pub fn add_one(n: i32) -> i32 {
        n + 1
    }
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// Use `RGB` color mode, the primary color
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Use `RGB` color mode, the secondary color
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange = 1,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Mix two primary color to create secondary color
    ///
    /// # Examples:
    /// ```
    /// use docs::kinds::*;
    /// use docs::utils::mix;
    ///
    /// let c = mix(PrimaryColor::Red, PrimaryColor::Blue).unwrap();
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Red => None,
                PrimaryColor::Yellow => Some(SecondaryColor::Orange),
                PrimaryColor::Blue => Some(SecondaryColor::Purple),
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Orange),
                PrimaryColor::Yellow => None,
                PrimaryColor::Blue => Some(SecondaryColor::Green),
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Purple),
                PrimaryColor::Yellow => Some(SecondaryColor::Green),
                PrimaryColor::Blue => None,
            }
        }
    }
}
