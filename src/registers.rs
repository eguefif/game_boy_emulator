#![allow(dead_code)]
#![allow(clippy::new_without_default)]

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0x00,
            b: 0xff,
            c: 0x13,
            d: 0x00,
            e: 0xc1,
            h: 0x84,
            l: 0x03,
            sp: 0xfffe,
        }
    }

    pub fn hl(&mut self) -> u16 {
        combine(self.h as u16, self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        (self.h, self.l) = split_u8(value);
    }
}

fn combine(high: u16, low: u16) -> u16 {
    (high << 8) | low
}

fn split_u8(value: u16) -> (u8, u8) {
    let high = (value >> 8) & 0xFF;
    let low = value & 0xFF;
    (high as u8, low as u8)
}

pub enum Target8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    SP,
}
