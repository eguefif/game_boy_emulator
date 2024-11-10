#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::debug_tools::handle_debug;
use crate::memorybus::MemoryBus;
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
        match opcode {
            0x0 => {}

            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            _ => {
                panic!("Opcode unknown: {}", opcode);
            }
        }
    }

    fn load<O: Copy, I: Copy>(&mut self, target: O, src: I)
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
}
