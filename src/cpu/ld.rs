use crate::cpu::read_write_cpu::{Source8, Target16, Target8};
use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::Cpu;

use super::registers::{test_carry_8, test_half_carry_8};

impl Cpu {
    pub fn load_imm16_sp(&mut self) {
        let low = self.reg.sp as u8;
        let high = (self.reg.sp >> 8) as u8;
        let loc = self.memory.fetch_next_word();
        self.memory.write_byte(loc, low);
        self.memory.write_byte(loc.wrapping_add(1), high);
    }

    pub fn load_hl_sp_imm8(&mut self) {
        let addend = self.memory.fetch_next_byte() as i8;
        let sp = self.reg.sp;
        let result = sp.wrapping_add(addend as i16 as u16);
        self.reg.set_hl(result);
        self.memory.tick();

        self.reg.set_flag(ZERO, false);
        self.reg.set_flag(N, false);
        let carry = test_carry_8(sp as u8, addend as u8);
        let half_carry = test_half_carry_8(sp as u8, addend as u8);
        self.reg.set_flag(CARRY, carry);
        self.reg.set_flag(HALF, half_carry);
    }

    pub fn load_sp_hl(&mut self) {
        self.reg.sp = self.reg.hl();
        self.memory.tick();
    }

    pub fn load16_imm<T: Copy>(&mut self, target: T)
    where
        Self: Target16<T>,
    {
        let value = self.memory.fetch_next_word();
        self.write16(target, value);
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
    use crate::debug_tools::handle_debug;

    use super::*;

    #[test]
    fn ldsp8_should_set_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xFF;
        let value: i8 = 1;
        cpu.reg.set_hl(0x0);
        cpu.memory.write_byte(cpu.memory.pc + 1, value as u8);
        set_first_instruction(0xF8, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn ldsp8_should_set_half_carry() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xF;
        let value: i8 = 1;
        cpu.reg.set_hl(0x0);
        cpu.memory.write_byte(cpu.memory.pc + 1, value as u8);
        set_first_instruction(0xF8, &mut cpu);

        cpu.step();

        assert!(cpu.reg.is_flag(HALF));
    }

    #[test]
    fn it_should_ld_sp_s8_neg_low_in_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xaabb;
        let value: i8 = -5;
        cpu.reg.set_hl(0x0);
        cpu.memory.write_byte(cpu.memory.pc + 1, value as u8);
        set_first_instruction(0xF8, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.hl(), 0xaabb - 5)
    }

    #[test]
    fn it_should_ld_hl_low_in_sp() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0x0;
        cpu.reg.set_hl(0xabcd);
        set_first_instruction(0xF9, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.sp, 0xabcd);
    }

    #[test]
    fn it_should_ld_sp_low_in_imm16() {
        let mut cpu = Cpu::new();
        cpu.reg.sp = 0xABCD;
        let loc = 0x10;
        cpu.memory.write_word(cpu.memory.pc + 1, loc);
        set_first_instruction(0x08, &mut cpu);

        cpu.step();

        assert_eq!(cpu.memory.fetch_byte(loc), 0xCD);
        assert_eq!(cpu.memory.fetch_byte(loc.wrapping_add(1)), 0xAB);
    }

    #[test]
    fn it_should_mov_c_to_b() {
        let mut cpu = Cpu::new();
        cpu.reg.c = 0xa;
        set_first_instruction(0x41, &mut cpu);

        cpu.step();

        assert_eq!(cpu.reg.b, 0xa);
    }

    #[test]
    fn it_should_mov_a_to_mem_hl() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0xa;

        let loc = 0x10;
        cpu.reg.set_hl(loc);
        set_first_instruction(0x77, &mut cpu);

        cpu.step();

        assert_eq!(cpu.memory.fetch_byte(loc), 0xa);
    }

    #[test]
    fn it_should_ld_d8_in_b() {
        let mut cpu = Cpu::new();
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xa);

        set_first_instruction(0x06, &mut cpu);
        cpu.reg.b = 0;

        cpu.step();

        assert_eq!(cpu.reg.b, 0xa)
    }

    #[test]
    fn it_should_ld_a_in_zero_page() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xE0, &mut cpu);
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xBD);
        cpu.reg.a = 0xa;

        cpu.step();

        assert_eq!(cpu.memory.read(0xFFBD), 0xa)
    }

    #[test]
    fn it_should_ld_zero_page_in_a() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xF0, &mut cpu);
        cpu.memory.write_byte(cpu.memory.pc + 1, 0xBD);
        cpu.memory.write_byte(0xFFBD, 0xb);
        cpu.reg.a = 0x0;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xb)
    }

    #[test]
    fn it_should_ld_a_in_zero_pagec() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xE2, &mut cpu);
        cpu.reg.a = 0xa;
        cpu.reg.c = 0xBD;

        cpu.step();

        assert_eq!(cpu.memory.read(0xFFBD), 0xa)
    }

    #[test]
    fn it_should_ld_zero_pagec_in_a() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xF2, &mut cpu);
        cpu.memory.write_byte(0xFFBD, 0xb);
        cpu.reg.a = 0x0;
        cpu.reg.c = 0xBD;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xb)
    }

    #[test]
    fn it_should_ld_addr_a16_in_a() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xFA, &mut cpu);
        cpu.memory.write_word(cpu.memory.pc + 1, 0xFFBD);
        cpu.memory.write_byte(0xFFBD, 0xb);
        cpu.reg.a = 0x0;

        cpu.step();

        assert_eq!(cpu.reg.a, 0xb)
    }

    #[test]
    fn it_should_ld_a_in_addr16() {
        let mut cpu = Cpu::new();

        set_first_instruction(0xEA, &mut cpu);
        cpu.memory.write_word(cpu.memory.pc + 1, 0xFFBD);
        cpu.reg.a = 0xa;

        cpu.step();

        assert_eq!(cpu.memory.read(0xFFBD), 0xa)
    }

    #[test]
    fn it_should_ld_imm16_in_bc() {
        let mut cpu = Cpu::new();

        set_first_instruction(0x01, &mut cpu);
        cpu.memory.write_word(cpu.memory.pc + 1, 0xffbd);

        cpu.step();

        assert_eq!(cpu.reg.bc(), 0xffbd)
    }

    pub fn set_first_instruction(value: u8, cpu: &mut Cpu) {
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, value);
    }
}
