use std::cmp::Ordering;
use std::fmt;

use super::config::Tile;

#[derive(Copy, Clone)]
pub struct Object {
    pub x: u8,
    pub y: u8,
    pub index: u8,
    pub flags: u8,
    pub oam_position: u8,
}

impl Object {
    pub fn new(x: u8, y: u8, index: u8, flags: u8, oam_position: u8) -> Object {
        Object {
            x,
            y,
            index,
            flags,
            oam_position,
        }
    }
}

pub fn flip_tile_if_flag_16(tile: Tile, tile2: Tile, flags: u8) -> (Tile, Tile) {
    if flags & 0b0100_0000 == 0 && flags & 0b0010_0000 == 0 {
        return (tile, tile2);
    }
    let mut new_sprite1 = [[0; 8]; 8];
    let mut new_sprite2 = [[0; 8]; 8];
    let mut tmp;

    if flags & 0b0100_0000 != 0 {
        for y in 0..8 {
            new_sprite1[y] = tile2[7 - y];
            new_sprite2[7 - y] = tile[y];
        }
    }
    if flags & 0b0010_0000 != 0 {
        for y in 0..8 {
            new_sprite1[y] = flip_x(tile[y]);
            new_sprite2[y] = flip_x(tile2[y]);
        }
    }
    if flags & 0b0100_0000 != 0 && flags & 0b0010_0000 != 0 {
        new_sprite1 = [[0; 8]; 8];
        new_sprite2 = [[0; 8]; 8];
        for y in 0..8 {
            new_sprite1[y] = flip_x(tile[y]);
            new_sprite2[y] = flip_x(tile2[y]);
        }
        for y in 0..8 {
            tmp = new_sprite1[y];
            new_sprite1[y] = new_sprite2[7 - y];
            new_sprite2[7 - y] = tmp;
        }
    }

    (new_sprite1, new_sprite2)
}

pub fn flip_tile_if_flag(tile: Tile, flags: u8) -> Tile {
    if flags & 0b0100_0000 == 0 && flags & 0b0010_0000 == 0 {
        return tile;
    }
    let mut new_tile = [[0; 8]; 8];
    if flags & 0b0010_0000 != 0 {
        for y in 0..8 {
            new_tile[y] = flip_x(tile[y]);
        }
    } else if flags & 0b0100_0000 != 0 {
        for y in 0..8 {
            new_tile[y] = tile[7 - y];
        }
    }
    if flags & 0b100_0000 != 0 && flags & 0b10_0000 != 0 {
        new_tile = [[0; 8]; 8];
        for y in 0..8 {
            new_tile[y] = tile[7 - y];
        }

        for y in 0..8 {
            new_tile[y] = flip_x(new_tile[y]);
        }
    }
    new_tile
}

fn flip_x(row: [u8; 8]) -> [u8; 8] {
    let mut new_row = [0; 8];
    for x in 0..8 {
        new_row[x] = row[7 - x];
    }
    new_row
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
    fn lt(&self, other: &Self) -> bool {
        if self.x != other.x {
            self.x < other.x
        } else {
            self.oam_position < other.oam_position
        }
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
        let o1 = Object::new(1, 2, 5, 5, 3);
        let o2 = Object::new(2, 2, 5, 5, 5);

        assert!(o1 < o2)
    }

    #[test]
    fn it_should_be_greater_than() {
        let o1 = Object::new(3, 2, 5, 5, 3);
        let o2 = Object::new(2, 2, 5, 5, 5);

        assert!(o1 > o2)
    }

    #[test]
    fn it_should_be_less_if_eq() {
        let o1 = Object::new(2, 2, 5, 5, 3);
        let o2 = Object::new(2, 2, 5, 5, 5);

        assert!(o1 < o2)
    }
}
