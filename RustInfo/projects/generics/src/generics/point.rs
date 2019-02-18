/* Use generics to define struct */
#[derive(Debug)]
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

// Methods for Point
impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &U {
        &self.y
    }
}

// Methods for the specific type
impl Point<f64, f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
