use crate::cpu::registers::Flags::{CARRY, HALF, N, ZERO};
use crate::cpu::Cpu;

impl Cpu {
    pub fn rlca(&mut self) {
        let a = self.reg.a;
        let carry = a >> 7 & 0b_0000_0001;
        self.reg.a = a.rotate_left(1);
        self.set_ra_flags(carry);
    }

    pub fn rla(&mut self) {
        let a = self.reg.a;
        let carry = a >> 7 & 0b_0000_0001;
        self.reg.a = (a << 1) | self.reg.is_flag(CARRY) as u8;
        self.set_ra_flags(carry);
    }

    pub fn rrca(&mut self) {
        let a = self.reg.a;
        let carry = a & 0b_0000_0001;
        self.reg.a = a.rotate_right(1);
        self.set_ra_flags(carry);
    }

    pub fn rra(&mut self) {
        let a = self.reg.a;
        let carry = a & 0b_0000_0001;
        self.reg.a = (a >> 1) | self.reg.is_flag(CARRY) as u8;
        self.set_ra_flags(carry);
    }

    fn set_ra_flags(&mut self, value: u8) {
        self.reg.set_flag(ZERO, false);
        self.reg.set_flag(HALF, false);
        self.reg.set_flag(N, false);
        self.reg.set_flag(CARRY, value != 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_rla() {
        let mut cpu = Cpu::new();
        cpu.reg.a = 0b_0101_0010;
        let pc = cpu.memory.pc;
        cpu.reg.set_flag(CARRY, true);
        cpu.memory.write_byte(pc, 0x17);

        cpu.step();

        assert_eq!(cpu.reg.a, 0b1010_0101);
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
