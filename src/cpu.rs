#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use crate::debug_tools::handle_debug;
use crate::execute::Addr;
use crate::execute::Addr::{BC, DE, HL};
use crate::memorybus::MemoryBus;
use crate::registers::Reg8;
use crate::registers::Reg8::{A, B, C, D, E, H, L};
use crate::registers::Registers;

pub struct Cpu {
    pub reg: Registers,
    pub memory: MemoryBus,
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
        handle_debug(opcode, self);
        self.execute(opcode);
    }
}
