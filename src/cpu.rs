#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use crate::debug_tools::handle_debug;
use crate::execute::Addr;
use crate::execute::Addr::{BC, DE, HL};
use crate::memorybus::MemoryBus;
use crate::read_write_cpu::{Source8, Target8};
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
        set_instruction(0x41, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.b, 0xa);
    }

    #[test]
    fn it_should_mov_a_to_mem_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xa;

        let loc = 0x10;
        cpu.reg.set_hl(loc);
        set_instruction(0x77, &mut cpu);

        cpu.step();

        assert_eq!(cpu.memory.fetch_byte(loc), 0xa);
    }

    #[test]
    fn it_should_ld_d8_in_b() {
        let mut cpu = Cpu::new();
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xa);

        set_instruction(0x06, &mut cpu);
        cpu.reg.b = 0;

        cpu.step();

        assert_eq!(cpu.reg.b, 0xa)
    }

    fn set_instruction(value: u8, cpu: &mut Cpu) {
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, value);
    }
}
