#![allow(dead_code)]
#![allow(clippy::new_without_default)]

pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0x0100,
            bc: 0xff13,
            de: 0x00c1,
            hl: 0x8403,
            sp: 0xfffe,
        }
    }
}
