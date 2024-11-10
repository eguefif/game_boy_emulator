#![allow(unused_imports)]
use crate::cpu::registers::Reg16::{BC, DE, HL, SP};
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::Cpu;

#[derive(Copy, Clone, Debug)]
pub enum Addr {
    BC,
    DE,
    HL,
    HLI,
    HLD,
    Imm16,
    ZeroPage,
    ZeroPageC,
}

#[derive(Copy, Clone, Debug)]
pub struct Imm8;

impl Cpu {
    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x0 => {}
            //******* Arithmetic Logic Unit (ALU)
            0xE8 => self.add_sp_s8(),

            0x04 => self.inc(B),
            0x14 => self.inc(D),
            0x24 => self.inc(H),
            0x34 => self.inc(Addr::HL),
            0x0C => self.inc(C),
            0x1C => self.inc(E),
            0x2C => self.inc(L),
            0x3C => self.inc(A),

            0x05 => self.dec(B),
            0x15 => self.dec(D),
            0x25 => self.dec(H),
            0x35 => self.dec(Addr::HL),
            0x0D => self.dec(C),
            0x1D => self.dec(E),
            0x2D => self.dec(L),
            0x3D => self.dec(A),

            0x80 => self.add(B),
            0x81 => self.add(C),
            0x82 => self.add(D),
            0x83 => self.add(E),
            0x84 => self.add(H),
            0x85 => self.add(L),
            0x86 => self.add(Addr::HL),
            0x87 => self.add(A),
            0x88 => self.addc(B),
            0x89 => self.addc(C),
            0x8a => self.addc(D),
            0x8b => self.addc(E),
            0x8c => self.addc(H),
            0x8d => self.addc(L),
            0x8e => self.addc(Addr::HL),
            0x8f => self.addc(A),

            0xC6 => self.add(Imm8),
            0xD6 => self.sub(Imm8),
            0xE6 => self.and(Imm8),
            0xF6 => self.or(Imm8),
            0xCE => self.addc(Imm8),
            0xDE => self.subc(Imm8),
            0xEE => self.xor(Imm8),
            0xFE => self.cp(Imm8),

            0x90 => self.sub(B),
            0x91 => self.sub(C),
            0x92 => self.sub(D),
            0x93 => self.sub(E),
            0x94 => self.sub(H),
            0x95 => self.sub(L),
            0x96 => self.sub(Addr::HL),
            0x97 => self.sub(A),
            0x98 => self.subc(B),
            0x99 => self.subc(C),
            0x9a => self.subc(D),
            0x9b => self.subc(E),
            0x9c => self.subc(H),
            0x9d => self.subc(L),
            0x9e => self.subc(Addr::HL),
            0x9f => self.subc(A),

            0xA0 => self.and(B),
            0xA1 => self.and(C),
            0xA2 => self.and(D),
            0xA3 => self.and(E),
            0xA4 => self.and(H),
            0xA5 => self.and(L),
            0xA6 => self.and(Addr::HL),
            0xA7 => self.and(A),
            0xA8 => self.xor(B),
            0xA9 => self.xor(C),
            0xAA => self.xor(D),
            0xAB => self.xor(E),
            0xAC => self.xor(H),
            0xAD => self.xor(L),
            0xAE => self.xor(Addr::HL),
            0xAF => self.xor(A),

            0xB0 => self.or(B),
            0xB1 => self.or(C),
            0xB2 => self.or(D),
            0xB3 => self.or(E),
            0xB4 => self.or(H),
            0xB5 => self.or(L),
            0xB6 => self.or(Addr::HL),
            0xB7 => self.or(A),
            0xB8 => self.cp(B),
            0xB9 => self.cp(C),
            0xBA => self.cp(D),
            0xBB => self.cp(E),
            0xBC => self.cp(H),
            0xBD => self.cp(L),
            0xBE => self.cp(Addr::HL),
            0xBF => self.cp(A),

            //******* Flow control
            0x76 => self.halt(),

            //***** Load section
            // Load sp
            0x08 => self.load_imm16_sp(),
            0xF8 => self.load_hl_sp_imm8(),
            0xF9 => self.load_sp_hl(),
            //
            // ld imm16
            0x01 => self.load16_imm(BC),
            0x11 => self.load16_imm(DE),
            0x21 => self.load16_imm(HL),
            0x31 => self.load16_imm(SP),

            // Load addr16
            0x02 => self.load(Addr::BC, A),
            0x12 => self.load(Addr::DE, A),
            0x22 => self.load(Addr::HLI, A),
            0x32 => self.load(Addr::HLD, A),
            0x0A => self.load(A, Addr::BC),
            0x1A => self.load(A, Addr::DE),
            0x2A => self.load(A, Addr::HLI),
            0x3A => self.load(A, Addr::HLD),

            // Load imm8
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
            0xE2 => self.load(Addr::ZeroPageC, A),
            0xF2 => self.load(A, Addr::ZeroPageC),

            //Ld a16
            0xEA => self.load(Addr::Imm16, A),
            0xFA => self.load(A, Addr::Imm16),

            //Ld regular
            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            0x42 => self.load(B, D),
            0x43 => self.load(B, E),
            0x44 => self.load(B, H),
            0x45 => self.load(B, L),
            0x46 => self.load(B, Addr::HL),
            0x47 => self.load(B, A),
            0x48 => self.load(C, B),
            0x49 => self.load(C, C),
            0x4a => self.load(C, D),
            0x4b => self.load(C, E),
            0x4c => self.load(C, H),
            0x4d => self.load(C, L),
            0x4e => self.load(C, Addr::HL),
            0x4f => self.load(C, A),

            0x50 => self.load(D, B),
            0x51 => self.load(D, C),
            0x52 => self.load(D, D),
            0x53 => self.load(D, E),
            0x54 => self.load(D, H),
            0x55 => self.load(D, L),
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
