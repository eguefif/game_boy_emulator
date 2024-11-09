const ROM_B1_START: u16 = 0;
const ROM_B1_END: u16 = 0x7FFF;
const ROM_B1_SIZE: u16 = ROM_B1_END + 1;

pub struct MemoryBus {
    rom: [u8; ROM_B1_SIZE as usize],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            rom: [0; ROM_B1_SIZE as usize],
        }
    }
}
