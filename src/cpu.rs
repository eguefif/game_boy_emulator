#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use crate::cpu::execute::Addr;
use crate::cpu::execute::Addr::{BC, DE, HL};
use crate::cpu::registers::Reg8;
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::registers::Registers;
use crate::debug_tools::handle_debug;
use crate::memorybus::MemoryBus;

pub mod alu;
pub mod execute;
pub mod flow;
pub mod ld;
pub mod read_write_cpu;
pub mod registers;

pub struct Cpu {
    pub reg: Registers,
    pub memory: MemoryBus,
    halted: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            memory: MemoryBus::new(),
            halted: false,
        }
    }

    pub fn step(&mut self) {
        if !self.halted {
            let opcode = self.memory.fetch_next_byte();
            handle_debug(opcode, self);
            self.execute(opcode);
        }
    }
}
