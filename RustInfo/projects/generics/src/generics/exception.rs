#[derive(Debug)]
pub struct ImportantExcept<'a> {
    pub name: &'a str,
    pub except: &'a str,
}

// Methods lifetime
impl<'a> ImportantExcept<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn except_and_announce(&self, ann: &str) -> &str {
        // According to the second and third rule of lifetime elision
        // the output lifetime can be inferred as &'a self
        println!("Attention please: {}", ann);
        self.except
    }
}

impl <'a> ImportantExcept<'a> {
    pub fn multi_args(&mut self, ann: &str, newName: &'a str) -> &str {
        println!("Change exception with lifetime ignore");
        println!("Announcement: {}", ann);
        self.name = newName;
        self.name
    }
}
