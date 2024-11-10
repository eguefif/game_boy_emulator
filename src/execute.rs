#![allow(unused_imports)]
use crate::cpu::Cpu;
use crate::registers::Reg8::{A, B, C, D, E, H, L};

#[derive(Copy, Clone, Debug)]
pub enum Addr {
    BC,
    DE,
    HL,
    HLI,
    HLD,
    Imm8,
    ZeroPage,
}

#[derive(Copy, Clone, Debug)]
pub struct Imm8;

impl Cpu {
    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x0 => {}

            // Load
            0x02 => self.load(Addr::BC, A),
            0x12 => self.load(Addr::DE, A),
            0x22 => self.load(Addr::HLI, A),
            0x32 => self.load(Addr::HLD, A),
            0x0A => self.load(A, Addr::BC),
            0x1A => self.load(A, Addr::DE),
            0x2A => self.load(A, Addr::HLI),
            0x3A => self.load(A, Addr::HLD),

            0x06 => self.load(B, Imm8),
            0x16 => self.load(D, Imm8),
            0x26 => self.load(H, Imm8),
            0x36 => self.load(Addr::HL, Imm8),
            0x0E => self.load(C, Imm8),
            0x1E => self.load(E, Imm8),
            0x2E => self.load(L, Imm8),
            0x3E => self.load(A, Imm8),

            //Ld ZeroPage
            0xE0 => self.load(Addr::ZeroPage, A),
            0xF0 => self.load(A, Addr::ZeroPage),

            //Ld regular
            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            0x42 => self.load(B, D),
            0x43 => self.load(B, E),
            0x44 => self.load(B, H),
            0x45 => self.load(B, L),
            0x46 => self.load(B, Addr::HL),
            0x47 => self.load(B, A),
            0x48 => self.load(B, B),
            0x49 => self.load(C, C),
            0x4a => self.load(C, D),
            0x4b => self.load(C, E),
            0x4c => self.load(C, H),
            0x4d => self.load(C, L),
            0x4e => self.load(C, Addr::HL),
            0x4f => self.load(C, A),

            0x50 => self.load(D, A),
            0x51 => self.load(D, A),
            0x52 => self.load(D, A),
            0x53 => self.load(D, A),
            0x54 => self.load(D, A),
            0x55 => self.load(D, A),
            0x56 => self.load(D, Addr::HL),
            0x57 => self.load(D, A),
            0x58 => self.load(E, B),
            0x59 => self.load(E, C),
            0x5a => self.load(E, D),
            0x5b => self.load(E, E),
            0x5c => self.load(E, H),
            0x5d => self.load(E, L),
            0x5e => self.load(E, Addr::HL),
            0x5f => self.load(E, A),

            0x60 => self.load(H, B),
            0x61 => self.load(H, C),
            0x62 => self.load(H, D),
            0x63 => self.load(H, E),
            0x64 => self.load(H, H),
            0x65 => self.load(H, L),
            0x66 => self.load(H, Addr::HL),
            0x67 => self.load(H, A),
            0x68 => self.load(L, B),
            0x69 => self.load(L, C),
            0x6a => self.load(L, D),
            0x6b => self.load(L, E),
            0x6c => self.load(L, H),
            0x6d => self.load(L, L),
            0x6e => self.load(L, Addr::HL),
            0x6f => self.load(L, A),

            0x70 => self.load(Addr::HL, B),
            0x71 => self.load(Addr::HL, C),
            0x72 => self.load(Addr::HL, D),
            0x73 => self.load(Addr::HL, E),
            0x74 => self.load(Addr::HL, H),
            0x75 => self.load(Addr::HL, L),
            0x76 => panic!("to do halt"),
            0x77 => self.load(Addr::HL, A),
            0x78 => self.load(A, B),
            0x79 => self.load(A, C),
            0x7a => self.load(A, D),
            0x7b => self.load(A, E),
            0x7c => self.load(A, H),
            0x7d => self.load(A, L),
            0x7e => self.load(A, Addr::HL),
            0x7f => self.load(A, A),

            _ => {
                panic!("Opcode unknown: {}", opcode);
            }
        }
    }
}
