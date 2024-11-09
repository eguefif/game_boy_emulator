#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::memorybus::MemoryBus;
use crate::registers::Registers;

pub struct Cpu {
    reg: Registers,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
        }
    }
    pub fn step(&mut self, memory: &mut MemoryBus) {
        let opcode = memory.fetch_next_byte();
        match opcode {
            0x0 => {}
            _ => {
                println!("Opcode unknown: {}", opcode);
                panic!();
            }
        }
    }
}
