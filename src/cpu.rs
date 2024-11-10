#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

use crate::debug_tools::handle_debug;
use crate::memorybus::MemoryBus;
use crate::registers::Addr;
use crate::registers::Addr::{BC, DE, HL};
use crate::registers::Reg8;
use crate::registers::Reg8::{A, B, C, D, E, H, L};
use crate::registers::Registers;

pub trait Source8<T: Copy> {
    fn read(&mut self, src: T) -> u8;
}

pub trait Target8<T: Copy> {
    fn write(&mut self, src: T, value: u8);
}

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

impl Source8<Reg8> for Cpu {
    fn read(&mut self, src: Reg8) -> u8 {
        match src {
            A => self.reg.a,
            B => self.reg.b,
            C => self.reg.c,
            D => self.reg.d,
            E => self.reg.e,
            H => self.reg.h,
            L => self.reg.l,
        }
    }
}

impl Target8<Reg8> for Cpu {
    fn write(&mut self, src: Reg8, value: u8) {
        match src {
            A => self.reg.a = value,
            B => self.reg.b = value,
            C => self.reg.c = value,
            D => self.reg.d = value,
            E => self.reg.e = value,
            H => self.reg.h = value,
            L => self.reg.l = value,
        }
    }
}

impl Source8<Addr> for Cpu {
    fn read(&mut self, src: Addr) -> u8 {
        let addr = match src {
            Addr::BC => self.reg.bc(),
            Addr::DE => self.reg.de(),
            Addr::HL => self.reg.hl(),
        };

        self.memory.fetch_byte(addr)
    }
}

impl Target8<Addr> for Cpu {
    fn write(&mut self, src: Addr, value: u8) {
        let addr = match src {
            Addr::BC => self.reg.bc(),
            Addr::DE => self.reg.de(),
            Addr::HL => self.reg.hl(),
        };

        self.memory.write_byte(addr, value)
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
