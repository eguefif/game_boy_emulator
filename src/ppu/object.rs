use std::fmt;

#[derive(Copy, Clone)]
pub struct Object {
    pub x: u8,
    pub y: u8,
    pub index: u8,
    pub flags: u8,
}

impl Object {
    pub fn new(x: u8, y: u8, index: u8, flags: u8) -> Object {
        Object { x, y, index, flags }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Object: x -> {}, y -> {}, index -> {}, flags {:0>8b}",
            self.x, self.y, self.index, self.flags
        )
    }
}
