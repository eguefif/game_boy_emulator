use crate::cpu::Cpu;

impl Cpu {
    pub fn halt(&mut self) {
        self.halted = true;
    }
}
