use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::Cpu;

use crate::cpu::read_write_cpu::{Source8, Target8};
use std::num::Wrapping;

use crate::cpu::registers::test_carry_8;

use super::registers::test_half_carry_8;

impl Cpu {
    pub fn add<S: Copy>(&mut self, source: S)
    where
        Self: Source8<S>,
    {
        let acc = self.reg.a;
        let addend = self.read(source);
        let res = acc.wrapping_add(addend);

        self.reg.a = res;
        self.set_add_flag(acc, addend, res);
    }

    fn set_add_flag(&mut self, acc: u8, addend: u8, res: u8) {
        let carry = test_carry_8(acc, addend);
        let half = test_half_carry_8(acc, addend);

        self.reg.set_flag(ZERO, res == 0);
        self.reg.set_flag(HALF, half);
        self.reg.set_flag(CARRY, carry);
        self.reg.set_flag(N, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        cpu.reg.f = 0b_1101_0000;

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
