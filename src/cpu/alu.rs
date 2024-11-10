use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::Cpu;

use crate::cpu::read_write_cpu::{Source8, Target8};

use crate::cpu::registers::test_carry_8;

use super::registers::test_half_carry_8;

impl Cpu {
    pub fn and<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let value = self.read(source);
        let acc = self.reg.a;
        self.reg.a = value & acc;
        self.reg.set_flag(ZERO, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(HALF, true);
        self.reg.set_flag(CARRY, false);
    }

    pub fn xor<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let value = self.read(source);
        let acc = self.reg.a;
        self.reg.a = value ^ acc;
        self.reg.set_flag(ZERO, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(HALF, false);
        self.reg.set_flag(CARRY, false);
    }

    pub fn or<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let value = self.read(source);
        let acc = self.reg.a;
        self.reg.a = value | acc;
        self.reg.set_flag(ZERO, self.reg.a == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(HALF, false);
        self.reg.set_flag(CARRY, false);
    }

    pub fn cp<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let subend = self.read(source);
        self.get_sub_result(subend, 0);
    }

    pub fn sub<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let subend = self.read(source);
        self.reg.a = self.get_sub_result(subend, 0);
    }

    pub fn subc<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let subend = self.read(source);
        self.reg.a = self.get_sub_result(subend, 1);
    }

    pub fn get_sub_result(&mut self, value: u8, carry: u8) -> u8 {
        let acc = self.reg.a;
        let res = acc.wrapping_sub(value).wrapping_sub(carry);

        let carry_f = (acc as u16) < (value as u16 + carry as u16);
        let half_f = (acc & 0x0F) < (value & 0x0F) + carry;

        self.reg.set_flag(ZERO, res == 0);
        self.reg.set_flag(HALF, half_f);
        self.reg.set_flag(CARRY, carry_f);
        self.reg.set_flag(N, true);
        res
    }

    pub fn add<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let acc = self.reg.a;
        let addend = self.read(source);
        let res = acc.wrapping_add(addend);

        self.reg.a = res;
        self.set_add_flag(acc, addend, res, 0);
    }

    pub fn addc<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let acc = self.reg.a;
        let addend = self.read(source);
        let mut carry_value = 0;
        if self.reg.is_flag(CARRY) {
            carry_value = 1
        }
        let res = acc.wrapping_add(addend).wrapping_add(carry_value);

        self.reg.a = res;
        self.set_add_flag(acc, addend, res, carry_value);
    }

    fn set_add_flag(&mut self, acc: u8, addend: u8, res: u8, carry_value: u8) {
        let carry = test_carry_8(acc, addend, carry_value);
        let half = test_half_carry_8(acc, addend, carry_value);

        self.reg.set_flag(ZERO, res == 0);
        self.reg.set_flag(HALF, half);
        self.reg.set_flag(CARRY, carry);
        self.reg.set_flag(N, false);
    }
}

#[cfg(test)]
mod tests {
    use crate::debug_tools::handle_debug;

    use super::*;

    #[test]
    fn it_should_adc_imm8() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xCE, &mut cpu);
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xa);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0xaa;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xaa + 0xa + 1);
    }

    #[test]
    fn it_should_sbc_imm8() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xDE, &mut cpu);
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xa);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0xaa;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xaa - 0xa - 1);
    }

    #[test]
    fn it_should_add_imm8() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xC6, &mut cpu);
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xa);
        cpu.reg.a = 0x5;

        cpu.step();

        assert_eq!(cpu.reg.a, 0x5 + 0xa);
    }

    #[test]
    fn it_should_xor() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xAE, &mut cpu);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0b_1111_1000;
        let loc = 0x10;
        cpu.memory.write_byte(loc, 0b_0100_1100);
        cpu.reg.set_hl(loc);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b_1011_0100);
    }

    #[test]
    fn it_should_or() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xB1, &mut cpu);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0b_1111_1000;
        cpu.reg.c = 0b_0100_1100;

        cpu.step();

        assert_eq!(cpu.reg.a, 0b_1111_1100);
    }

    #[test]
    fn it_should_and() {
        let mut cpu = Cpu::new();
        set_first_instruction(0xA0, &mut cpu);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0b_1111_1000;
        cpu.reg.b = 0b_0100_1111;

        cpu.step();

        assert_eq!(cpu.reg.a, 0b_0100_1000);
    }

    #[test]
    fn it_should_sub_0x9a() {
        let mut cpu = Cpu::new();
        set_first_instruction(0x9a, &mut cpu);
        cpu.reg.set_flag(CARRY, true);
        cpu.reg.a = 0xab;
        cpu.reg.d = 0x12;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xab - 0x12 - 1);
    }

    #[test]
    fn it_should_sub_0x90() {
        let mut cpu = Cpu::new();
        set_first_instruction(0x90, &mut cpu);
        cpu.reg.a = 0xab;
        cpu.reg.b = 0x12;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xab - 0x12);
    }

    #[test]
    fn it_should_sub_and_set_carry_with_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x11;
        let _ = cpu.get_sub_result(0x1, 1);

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_sub_and_set_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0;
        let _ = cpu.get_sub_result(0x1, 0);

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_sub_and_set_half_carry_with_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x1;
        let _ = cpu.get_sub_result(0x1, 1);

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_sub_and_set_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x10;
        let _ = cpu.get_sub_result(0x1, 0);

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_sub_and_set_n() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xAA;
        let _ = cpu.get_sub_result(0x5, 0);

        assert!(cpu.reg.is_flag(N));
    }

    #[test]
    fn it_should_sub() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xAA;
        let res = cpu.get_sub_result(0x5, 0);

        assert_eq!(res, 0xAA - 0x5);
    }
    #[test]
    fn it_should_addc_b_into_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xAD;
        cpu.reg.b = 0x3;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x88, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xAD + 0x3 + 0x1);
    }

    #[test]
    fn it_should_addc_b_into_a_and_unset_flag_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFD;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x88, &mut cpu);

        cpu.step();

        assert!(!cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_addc_b_into_a_and_unset_flag_h() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xD;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x88, &mut cpu);

        cpu.step();

        assert!(!cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_addc_b_into_a_and_set_flag_h() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xE;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1101_0000;

        set_first_instruction(0x88, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_add_b_into_a_and_unset_flag_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFE;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(!cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_add_b_into_a_and_set_flag_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xFF;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1110_0000;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(CARRY));
    }

    #[test]
    fn it_should_add_b_into_a_and_unset_flag_h() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xE;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(!cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_add_b_into_a_and_set_flag_h() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xF;
        cpu.reg.b = 0x1;
        cpu.reg.f = 0b_1101_0000;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_add_b_into_a_and_unset_flag_n() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0;
        cpu.reg.b = 0x0;
        cpu.reg.f = 0b_1111_0000;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(!cpu.reg.is_flag(N));
    }

    #[test]
    fn it_should_add_b_into_a_and_set_flag_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0x0;
        cpu.reg.b = 0x0;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(ZERO));
    }

    #[test]
    fn it_should_add_b_into_a() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xa;
        cpu.reg.b = 0x5;

        set_first_instruction(0x80, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.a, 0xa + 0x5);
    }

    fn set_first_instruction(value: u8, cpu: &mut Cpu) {
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, value);
    }
}
