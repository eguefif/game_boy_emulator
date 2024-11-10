#![allow(unused_imports)]
use crate::cpu::Cpu;
use crate::execute::{Addr, Imm8};
use crate::registers::Reg16;
use crate::registers::Reg16::{BC, DE, HL, SP};
use crate::registers::Reg8::{A, B, C, D, E, H, L};
use crate::registers::{combine, Reg8};

pub trait Source8<T: Copy> {
    fn read(&mut self, src: T) -> u8;
}

pub trait Target8<T: Copy> {
    fn write(&mut self, src: T, value: u8);
}

pub trait Target16<T: Copy> {
    fn write16(&mut self, src: T, value: u16);
}

impl Target16<Reg16> for Cpu {
    fn write16(&mut self, target: Reg16, value: u16) {
        match target {
            BC => self.reg.set_bc(value),
            DE => self.reg.set_de(value),
            HL => self.reg.set_hl(value),
            SP => self.reg.sp = value,
        }
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
            Addr::HLI => {
                let hl = self.reg.hl();
                self.reg.set_hl(hl.wrapping_add(1));
                hl
            }
            Addr::HLD => {
                let hl = self.reg.hl();
                self.reg.set_hl(hl.wrapping_sub(1));
                hl
            }
            Addr::Imm16 => self.memory.fetch_next_word(),
            Addr::ZeroPage => {
                let low = self.memory.fetch_next_byte();
                combine(0xFF, low as u16)
            }
            Addr::ZeroPageC => {
                let low = self.reg.c;
                combine(0xFF, low as u16)
            }
        };

        self.memory.fetch_byte(addr)
    }
}

impl Target8<Addr> for Cpu {
    fn write(&mut self, target: Addr, value: u8) {
        let addr = match target {
            Addr::BC => self.reg.bc(),
            Addr::DE => self.reg.de(),
            Addr::HL => self.reg.hl(),
            Addr::HLI => {
                let hl = self.reg.hl();
                self.reg.set_hl(hl.wrapping_add(1));
                hl
            }
            Addr::HLD => {
                let hl = self.reg.hl();
                self.reg.set_hl(hl.wrapping_sub(1));
                hl
            }
            Addr::Imm16 => self.memory.fetch_next_word(),
            Addr::ZeroPage => {
                let low = self.memory.fetch_next_byte();
                combine(0xFF, low as u16)
            }
            Addr::ZeroPageC => {
                let low = self.reg.c;
                combine(0xFF, low as u16)
            }
        };

        self.memory.write_byte(addr, value)
    }
}

impl Source8<Imm8> for Cpu {
    fn read(&mut self, _src: Imm8) -> u8 {
        self.memory.fetch_next_byte()
    }
}

impl Target8<Imm8> for Cpu {
    fn write(&mut self, _tar: Imm8, value: u8) {
        self.memory.write_byte(self.memory.pc, value);
    }
}
