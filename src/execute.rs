use crate::cpu::Cpu;
use crate::registers::Addr::{BC, DE, HL};
use crate::registers::Reg8::{A, B, C, D, E, H, L};

impl Cpu {
    pub fn execute(&mut self, opcode: u8) {
        match opcode {
            0x0 => {}

            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            0x42 => self.load(B, D),
            0x43 => self.load(B, E),
            0x44 => self.load(B, H),
            0x45 => self.load(B, L),
            0x46 => self.load(B, HL),
            0x47 => self.load(B, A),
            0x48 => self.load(B, B),
            0x49 => self.load(C, C),
            0x4a => self.load(C, D),
            0x4b => self.load(C, E),
            0x4c => self.load(C, H),
            0x4d => self.load(C, L),
            0x4e => self.load(C, HL),
            0x4f => self.load(C, A),

            0x50 => self.load(D, A),
            0x51 => self.load(D, A),
            0x52 => self.load(D, A),
            0x53 => self.load(D, A),
            0x54 => self.load(D, A),
            0x55 => self.load(D, A),
            0x56 => self.load(D, HL),
            0x57 => self.load(D, A),
            0x58 => self.load(E, B),
            0x59 => self.load(E, C),
            0x5a => self.load(E, D),
            0x5b => self.load(E, E),
            0x5c => self.load(E, H),
            0x5d => self.load(E, L),
            0x5e => self.load(E, HL),
            0x5f => self.load(E, A),

            0x60 => self.load(H, B),
            0x61 => self.load(H, C),
            0x62 => self.load(H, D),
            0x63 => self.load(H, E),
            0x64 => self.load(H, H),
            0x65 => self.load(H, L),
            0x66 => self.load(H, HL),
            0x67 => self.load(H, A),
            0x68 => self.load(L, B),
            0x69 => self.load(L, C),
            0x6a => self.load(L, D),
            0x6b => self.load(L, E),
            0x6c => self.load(L, H),
            0x6d => self.load(L, L),
            0x6e => self.load(L, HL),
            0x6f => self.load(L, A),

            0x70 => self.load(HL, B),
            0x71 => self.load(HL, C),
            0x72 => self.load(HL, D),
            0x73 => self.load(HL, E),
            0x74 => self.load(HL, H),
            0x75 => self.load(HL, L),
            0x76 => panic!("to do halt"),
            0x77 => self.load(HL, A),
            0x78 => self.load(A, B),
            0x79 => self.load(A, C),
            0x7a => self.load(A, D),
            0x7b => self.load(A, E),
            0x7c => self.load(A, H),
            0x7d => self.load(A, L),
            0x7e => self.load(A, HL),
            0x7f => self.load(A, A),

            _ => {
                panic!("Opcode unknown: {}", opcode);
            }
        }
    }
}
