use std::cmp::Ordering;
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

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Object {}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_should_be_lesser_than() {
        let o1 = Object::new(1, 2, 5, 5);
        let o2 = Object::new(2, 2, 5, 5);

        assert!(o1 < o2)
    }

    #[test]
    fn it_should_be_greater_than() {
        let o1 = Object::new(3, 2, 5, 5);
        let o2 = Object::new(2, 2, 5, 5);

        assert!(o1 > o2)
    }

    #[test]
    fn it_should_be_equal() {
        let o1 = Object::new(2, 2, 5, 5);
        let o2 = Object::new(2, 2, 5, 5);

        assert!(o1 == o2)
    }
}
