#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use crate::debug_tools::handle_debug;
use crate::memorybus::MemoryBus;
use crate::read_write_cpu::{Source8, Target8};
use crate::registers::Addr;
use crate::registers::Addr::{BC, DE, HL};
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

    pub fn load<O: Copy, I: Copy>(&mut self, target: O, src: I)
    where
        Self: Target8<O> + Source8<I>,
    {
        let src_value = self.read(src);
        self.write(target, src_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_mov_c_to_b() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xa;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x41);

        cpu.step();

        assert_eq!(cpu.reg.b, 0xa);
    }

    #[test]
    fn it_should_mov_a_to_mem_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xa;

        let loc = 0x10;
        cpu.reg.set_hl(loc);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x77);

        cpu.step();

        assert_eq!(cpu.memory.fetch_byte(loc), 0xa);
    }
}
