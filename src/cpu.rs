#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use registers::Reg16;

use crate::cpu::execute::Addr;
use crate::cpu::execute::Addr::{BC, DE, HL};
use crate::cpu::registers::Reg8;
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::registers::Registers;
use crate::debug_tools::handle_debug;
use crate::memorybus::MemoryBus;

pub mod alu;
pub mod bit_operations;
pub mod execute;
pub mod flow;
pub mod interrupt;
pub mod ld;
pub mod read_write_cpu;
pub mod registers;

pub struct Cpu {
    pub reg: Registers,
    pub memory: MemoryBus,
    ime: bool,
    prepare_ime: bool,
    halted: bool,
    nop_counter: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            memory: MemoryBus::new(),
            prepare_ime: false,
            ime: false,
            halted: false,
            nop_counter: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        let ime = self.ime;
        if self.prepare_ime {
            self.ime = !self.ime;
            self.prepare_ime = false;
        }
        if self.halted {
            self.memory.tick();
        }
        if self.halted && ime && self.memory.interrupt.should_interrupt() {
            self.halted = false;
        } else if ime && self.memory.interrupt.should_interrupt() {
            self.handle_interrupt();
        } else if !self.halted {
            let opcode = self.memory.fetch_next_byte();
            if opcode == 0 {
                self.nop_counter += 1;
                if self.nop_counter > 15 {
                    return false;
                }
            }
            handle_debug(opcode, self);
            self.execute(opcode);
        }
        true
    }

    fn handle_interrupt(&mut self) {
        println!("entering interrupt handler");
        self.memory.tick();
        self.memory.tick();
        self.ime = false;
        let addr = self.memory.interrupt.get_interrupt_addr();
        self.push(Reg16::SP);
        self.memory.pc = addr;
    }
}
