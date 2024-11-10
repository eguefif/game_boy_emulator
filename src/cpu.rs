#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::memorybus::MemoryBus;
use crate::registers::Registers;
use crate::registers::Target8;

pub struct Cpu {
    reg: Registers,
    memory: MemoryBus,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            memory: MemoryBus::new(),
        }
    }
    pub fn step(&mut self) {
        let opcode = self.memory.fetch_next_byte();
        match opcode {
            0x0 => {}

            0x40 => self.ld(B, B),
            0x41..=0x7c => self.regular_ld(opcode),
            _ => {
                panic!("Opcode unknown: {}", opcode);
            }
        }
    }
}
