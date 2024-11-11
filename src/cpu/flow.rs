use crate::cpu::registers::Flags::CARRY;
use crate::cpu::Cpu;

impl Cpu {
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
}
