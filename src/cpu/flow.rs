use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::registers::Reg16;
use crate::cpu::registers::Reg16::{AF, BC, DE, HL, SP};
use crate::cpu::Cpu;

use crate::cpu::execute::{Condition, JpAddr};

use crate::cpu::registers::{combine, split_u16};

impl Cpu {
    pub fn call(&mut self, condition: Condition) {
        let addr = self.memory.fetch_next_word();
        if self.should_jump(condition) {
            self.make_call(addr);
        }
    }

    fn make_call(&mut self, addr: u16) {
        let pc = self.memory.pc;
        self.make_push(pc);
        self.jump_to_addr(addr);
    }

    pub fn ret(&mut self, condition: Condition) {
        if condition != Condition::None {
            self.memory.tick()
        }
        if self.should_jump(condition) {
            self.make_ret();
        }
    }

    fn make_ret(&mut self) {
        let addr = self.make_pop();
        self.jump_to_addr(addr);
    }

    pub fn reti(&mut self) {
        panic!("Not imlemented yet: reti");
    }

    pub fn pop(&mut self, target: Reg16) {
        let value = self.make_pop();
        match target {
            AF => self.reg.set_af(value),
            BC => self.reg.set_bc(value),
            HL => self.reg.set_hl(value),
            DE => self.reg.set_de(value),
            SP => {}
        }
    }

    fn make_pop(&mut self) -> u16 {
        let lo = self.memory.fetch_byte(self.reg.sp);
        self.reg.inc_sp();
        let hi = self.memory.fetch_byte(self.reg.sp);
        self.reg.inc_sp();
        combine(hi as u16, lo as u16)
    }

    pub fn push(&mut self, target: Reg16) {
        let value = match target {
            AF => self.reg.af(),
            BC => self.reg.bc(),
            HL => self.reg.hl(),
            DE => self.reg.de(),
            SP => 0,
        };
        self.make_push(value);
    }

    fn make_push(&mut self, value: u16) {
        let (hi, lo) = split_u16(value);
        self.memory.tick();
        self.reg.dec_sp();
        self.memory.write_byte(self.reg.sp, lo);
        self.reg.dec_sp();
        self.memory.write_byte(self.reg.sp, hi);
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn scf(&mut self) {
        self.reg.set_flag(CARRY, true)
    }

    pub fn cpl(&mut self) {
        let a = self.reg.a;
        self.reg.a = !a;
    }

    pub fn ccf(&mut self) {
        let carry = self.reg.is_flag(CARRY);
        self.reg.set_flag(CARRY, carry);
    }

    pub fn jump(&mut self, condition: Condition, addr: JpAddr) {
        let addr = match addr {
            JpAddr::HL => self.reg.hl(),
            JpAddr::S8 => {
                let pc = self.memory.pc;
                let offset = self.memory.fetch_next_byte() as i8;
                pc.wrapping_add(offset as i16 as u16)
            }
            JpAddr::A16 => self.memory.fetch_next_word(),
        };
        if self.should_jump(condition) {
            self.jump_to_addr(addr);
        }
    }

    fn should_jump(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::NZ if !self.reg.is_flag(ZERO) => true,
            Condition::NZ => false,
            Condition::NC if !self.reg.is_flag(CARRY) => true,
            Condition::NC => false,
            Condition::Z if self.reg.is_flag(ZERO) => true,
            Condition::Z => false,
            Condition::C if self.reg.is_flag(CARRY) => true,
            Condition::C => false,
            Condition::None => true,
        }
    }

    fn jump_to_addr(&mut self, addr: u16) {
        self.memory.pc = addr;
        self.memory.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_jp_s8_no_carry() {
        let mut cpu = Cpu::new();
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x30);
        let value = -5;
        cpu.memory.write_byte(pc + 1, value as u8);
        cpu.reg.set_flag(CARRY, false);

        cpu.step();
        assert_eq!(cpu.memory.pc, pc + 1 - 5)
    }

    #[test]
    fn it_should_jp_s8_carry() {
        let mut cpu = Cpu::new();
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x38);
        cpu.memory.write_byte(pc + 1, 0x5);
        cpu.reg.set_flag(CARRY, true);

        cpu.step();
        assert_eq!(cpu.memory.pc, pc + 1 + 0x5)
    }

    #[test]
    fn it_should_not_jp_s8_carry() {
        let mut cpu = Cpu::new();
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0x38);
        cpu.memory.write_byte(pc + 1, 0x5);
        cpu.reg.set_flag(CARRY, false);

        cpu.step();
        assert_eq!(cpu.memory.pc, pc + 2)
    }

    #[test]
    fn it_should_jp_a16_zero() {
        let mut cpu = Cpu::new();
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xCA);
        cpu.memory.write_byte(pc + 1, 0x10);
        cpu.memory.write_byte(pc + 2, 0xa);
        cpu.reg.set_flag(ZERO, true);

        cpu.step();
        assert_eq!(cpu.memory.pc, 0xa10)
    }

    #[test]
    fn it_should_jp_hl() {
        let mut cpu = Cpu::new();
        let pc = cpu.memory.pc;
        cpu.memory.write_byte(pc, 0xE9);
        cpu.reg.set_hl(0xAA);

        cpu.step();
        assert_eq!(cpu.memory.pc, 0xAA)
    }
}
