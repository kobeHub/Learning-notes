/*define the rectangle struct and define methods*/
#[derive(Debug)]
pub struct Rectangle {
    width: u64,
    length: u64,
}

pub fn rectangle(width: u64, length: u64) -> Rectangle {
    Rectangle{width, length}
}

pub fn area(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.length
}

// methods impl rectangle
impl Rectangle {
    pub fn area(&self) -> u64 {
        self.width * self.length
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // associated function of the struct
    pub fn square(size: u64) -> Rectangle {
        Rectangle{width: size, length: size}
    }

    pub fn increase(&mut self, wid: u64, len: u64) {
        self.width += wid;
        self.length += len;
    }
}
