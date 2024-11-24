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
