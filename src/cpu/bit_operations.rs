use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::Cpu;

use crate::cpu::read_write_cpu::{Source8, Target8};

impl Cpu {
    pub fn swap(&mut self, opcode: u8) {
        let value = self.get_target(opcode);
        let new_value = value.rotate_left(4);
        self.set_target(opcode, new_value);
        self.reg.set_flag(ZERO, new_value == 0);
        self.reg.set_flag(CARRY, false);
        self.reg.set_flag(HALF, false);
        self.reg.set_flag(N, false);
    }
    pub fn set(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value |= 1 << (opcode >> 3 & 0b111);
        self.set_target(opcode, value);
    }

    pub fn reset(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value &= !(1 << (opcode >> 3 & 0b111));
        self.set_target(opcode, value);
    }

    pub fn bit(&mut self, opcode: u8) {
        let value = self.get_target(opcode);
        let result = value & 1 << (opcode >> 3 & 0b111);
        self.reg.set_flag(ZERO, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(HALF, true);
    }

    pub fn srl(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        let carry = value & 0b_0000_0001;
        value >>= 1;
        self.set_target(opcode, value);
        self.set_rotation_flags(carry, value);
    }

    pub fn sra(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        let carry = value & 0b_0000_0001;
        let bit7 = value & 0b_1000_0000;
        value = (value >> 1) | bit7;
        self.set_target(opcode, value);
        self.set_rotation_flags(carry, value);
    }

    pub fn sla(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        let carry = value >> 7 & 0b_0000_0001;
        value <<= 1;
        self.set_target(opcode, value);
        self.set_rotation_flags(carry, value);
    }
    // rlc and rlca
    pub fn rlc(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value = self.alu_rlc(value);
        self.set_target(opcode, value);
    }

    fn alu_rlc(&mut self, value: u8) -> u8 {
        let carry = value >> 7 & 0b_0000_0001;
        let res = value.rotate_left(1);

        self.set_rotation_flags(carry, res);
        res
    }

    pub fn rlca(&mut self) {
        let a = self.reg.a;
        self.reg.a = self.alu_rlc(a);
        self.reg.set_flag(ZERO, false);
    }

    // rl and rla
    pub fn rl(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value = self.alu_rl(value);
        self.set_target(opcode, value);
    }

    pub fn alu_rl(&mut self, value: u8) -> u8 {
        let carry = value >> 7 & 0b_0000_0001;
        let res = (value << 1) | self.reg.is_flag(CARRY) as u8;
        self.set_rotation_flags(carry, res);
        res
    }

    pub fn rla(&mut self) {
        let a = self.reg.a;
        self.reg.a = self.alu_rl(a);
        self.reg.set_flag(ZERO, false);
    }

    // rrc and rrca
    pub fn rrc(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value = self.alu_rrc(value);
        self.set_target(opcode, value);
    }

    pub fn alu_rrc(&mut self, value: u8) -> u8 {
        let carry = value & 0b_0000_0001;
        let res = value.rotate_right(1);
        self.set_rotation_flags(carry, res);
        res
    }

    pub fn rrca(&mut self) {
        let a = self.reg.a;
        self.reg.a = self.alu_rrc(a);
        self.reg.set_flag(ZERO, false);
    }

    // rr and rra
    pub fn rr(&mut self, opcode: u8) {
        let mut value = self.get_target(opcode);
        value = self.alu_rr(value);
        self.set_target(opcode, value);
    }

    pub fn alu_rr(&mut self, value: u8) -> u8 {
        let carry = value & 0b_0000_0001;
        let res = (value >> 1) | ((self.reg.is_flag(CARRY) as u8) << 7);
        self.set_rotation_flags(carry, res);
        res
    }

    pub fn rra(&mut self) {
        let a = self.reg.a;
        self.reg.a = self.alu_rr(a);
        self.reg.set_flag(ZERO, false);
    }

    fn set_rotation_flags(&mut self, value: u8, res: u8) {
        self.reg.set_flag(ZERO, res == 0);
        self.reg.set_flag(HALF, false);
        self.reg.set_flag(N, false);
        self.reg.set_flag(CARRY, value != 0);
    }

    fn get_target(&mut self, opcode: u8) -> u8 {
        match opcode & 0b_0000_0111 {
            0b0000 => self.reg.b,
            0b0001 => self.reg.c,
            0b0010 => self.reg.d,
            0b0011 => self.reg.e,
            0b0100 => self.reg.h,
            0b0101 => self.reg.l,
            0b0110 => {
                let hl = self.reg.hl();
                self.memory.fetch_byte(hl)
            }
            0b0111 => self.reg.a,
            _ => 0,
        }
    }

    fn set_target(&mut self, opcode: u8, value: u8) {
        match opcode & 0b_0000_0111 {
            0b0000 => self.reg.b = value,
            0b0001 => self.reg.c = value,
            0b0010 => self.reg.d = value,
            0b0011 => self.reg.e = value,
            0b0100 => self.reg.h = value,
            0b0101 => self.reg.l = value,
            0b0110 => {
                let hl = self.reg.hl();
                self.memory.write_byte(hl, value)
            }
            0b0111 => self.reg.a = value,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_swap() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_1001_0110;
        cpu.reg.set_flag(ZERO, false);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x37);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b_0110_1001);
        assert!(cpu.reg.is_flag(ZERO));
    }

    #[test]
    fn it_should_srl() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0b_1001_1001;
        cpu.reg.set_flag(ZERO, false);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x3B);

        cpu.step();

        assert_eq!(cpu.reg.e, 0b_0100_1100);
        assert!(cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_sra() {
        let mut cpu = Cpu::new();
        cpu.reg.h = 0b_1001_1001;
        cpu.reg.set_flag(ZERO, false);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x2C);

        cpu.step();

        assert_eq!(cpu.reg.h, 0b_1100_1100);
        assert!(cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_sla() {
        let mut cpu = Cpu::new();
        cpu.reg.l = 0b_1001_1011;
        cpu.reg.set_flag(ZERO, false);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x25);

        cpu.step();

        assert_eq!(cpu.reg.l, 0b_0011_0110);
        assert!(cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_bit_0_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0b_1111_1111;
        cpu.reg.set_flag(ZERO, true);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x41);

        cpu.step();

        assert!(!cpu.reg.is_flag(ZERO));
    }

    #[test]
    fn it_should_reset_2_a() {
        let mut cpu = Cpu::new();
        cpu.reg.e = 0b_1111_1111;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x93);

        cpu.step();

        assert_eq!(cpu.reg.e, 0b1111_1011);
    }

    #[test]
    fn it_should_set_3_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_0000_0000;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0xDF);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b0000_1000);
    }

    #[test]
    fn it_should_rl_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_0110_0010;
        cpu.reg.set_flag(CARRY, true);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x17);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b1100_0101);
    }

    #[test]
    fn it_should_rr_b() {
        let mut cpu = Cpu::new();
        cpu.reg.b = 0b_0110_0010;
        cpu.reg.set_flag(CARRY, true);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x18);

        cpu.step();

        assert_eq!(cpu.reg.b, 0b1011_0001);
    }

    #[test]
    fn it_should_rrc_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.set_hl(0x10);
        cpu.memory.write_byte(0x10, 0b_0100_0011);
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x0E);

        cpu.step();

        assert_eq!(cpu.memory.fetch_byte(0x10), 0b1010_0001);
    }

    #[test]
    fn it_should_rlc_c() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0b_1101_0010;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCB);
        cpu.memory.write_byte(pc + 1, 0x01);

        cpu.step();

        assert_eq!(cpu.reg.c, 0b1010_0101);
        assert!(cpu.reg.is_flag(CARRY));
        assert!(!cpu.reg.is_flag(ZERO));
    }

    #[test]
    fn it_should_rla() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_0101_0010;
        let pc = cpu.memory.pc;
        cpu.reg.set_flag(CARRY, true);
        cpu.memory.write_byte(pc, 0x17);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b1010_0101);
        assert!(!cpu.reg.is_flag(ZERO))
    }

    #[test]
    fn it_should_rlca() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_1001_0010;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x07);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b0010_0101);
        assert!(cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_rlca_no_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_0101_0010;
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x07);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b1010_0100);
        assert!(!cpu.reg.is_flag(CARRY));
    }
}
