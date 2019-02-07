/*define the rectangle struct and define methods*/
#[derive(Debug)]
pub struct Rectangle {
    width: u64,
    length: u64,
}

pub fn Rectangle(width: u64, length: u64) -> Rectangle {
    Rectangle{width, length}
}

pub fn Area(rectangle: &Rectangle) -> u64 {
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
    pub fn Square(size: u64) -> Rectangle {
        Rectangle{width: size, length: size}
    }

    pub fn Increase(&mut self, wid: u64, len: u64) {
        self.width += wid;
        self.length += len;
    }
}
