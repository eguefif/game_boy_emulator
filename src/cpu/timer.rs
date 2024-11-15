use crate::cpu::Cpu;

const DIV_FREQ: u128 = 16384;
pub const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

impl Cpu {
    pub fn handle_timer(&mut self) {
        self.handle_div();
        if self.is_tima_on() {
            self.handle_tima();
        }
    }

    fn handle_div(&mut self) {
        let modulo_div = self.memory.cycle % DIV_FREQ;
        if modulo_div < 7 {
            let value = self.memory.read(DIV) + 1;
            self.memory.write(DIV, value);
        }
    }

    fn is_tima_on(&mut self) -> bool {
        let tac = self.memory.read(TAC);
        tac >> 2 & 1 == 1
    }

    fn handle_tima(&mut self) {
        let freq_bits = self.memory.read(TAC) & 3;
        let freq = match freq_bits {
            0 => 4096,
            1 => 26144,
            2 => 65536,
            3 => 16384,
            _ => 0,
        };
        let modulo_div = self.memory.cycle % freq;
        if modulo_div < 7 {
            let tima = self.memory.read(TIMA);
            let (res, overflow) = tima.overflowing_add(1);
            if overflow {
                let tma = self.memory.read(TMA);
                self.memory.write(TIMA, tma);
                self.memory.interrupt.require_timer();
            } else {
                self.memory.write(TIMA, res);
            }
        }
    }
}
