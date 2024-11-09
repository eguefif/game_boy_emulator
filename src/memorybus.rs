#![allow(dead_code)]
#![allow(clippy::new_without_default)]

const ROM_B1_END: u16 = 0x3FFF;
const ROM_B1_SIZE: u16 = ROM_B1_END + 1;

const ROM_B2_START: u16 = 0x4000;
const ROM_B2_END: u16 = 0x7FFF;
const ROM_B2_SIZE: u16 = ROM_B2_END - ROM_B2_START + 1;

const MEM_MAX: u16 = 0xFFFF;

pub struct MemoryBus {
    rom1: [u8; ROM_B1_SIZE as usize],
    rom2: [u8; ROM_B2_SIZE as usize],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            rom1: [0; ROM_B1_SIZE as usize],
            rom2: [0; ROM_B2_SIZE as usize],
        }
    }

    pub fn fetch_next_byte(&mut self, at: u16) -> u8 {
        self.read(at)
    }

    pub fn write_byte(&mut self, at: u16, value: u8) {
        self.write(at, value)
    }

    fn read(&mut self, at: u16) -> u8 {
        let loc = at & MEM_MAX;

        match loc {
            0..=ROM_B1_END => self.rom1[loc as usize],
            ROM_B2_START..=ROM_B2_END => self.rom2[loc as usize],
            _ => {
                println!("Read: memory not handled: {}", loc);
                0
            }
        }
    }

    fn write(&mut self, at: u16, value: u8) {
        let loc = at & MEM_MAX;

        match loc {
            0..=ROM_B1_END => self.rom1[loc as usize] = value,
            ROM_B2_START..=ROM_B2_END => self.rom2[loc as usize] = value,
            _ => println!("Write: memory not handled: {}", loc),
        }
    }
}
