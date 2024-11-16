use crate::memorybus::MemoryBus;

const DIV_FREQ: u128 = 16384;
pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

impl MemoryBus {
    pub fn handle_timer(&mut self) {
        self.handle_div();
        if self.is_tima_on() {
            self.handle_tima();
        }
    }

    fn handle_div(&mut self) {
        let modulo_div = self.cycle % DIV_FREQ;
        if modulo_div == 0 {
            let (value, overflow) = self.read(DIV).overflowing_add(1);
            if overflow {
                self.interrupt.require_timer();
            }
            self.write(DIV, value);
        }
    }

    fn is_tima_on(&mut self) -> bool {
        let tac = self.read(TAC);
        tac >> 2 & 1 == 1
    }

    fn handle_tima(&mut self) {
        let freq_bits = self.read(TAC) & 3;
        let freq = match freq_bits {
            0 => 4096,
            1 => 26144,
            2 => 65536,
            3 => 16384,
            _ => 0,
        };
        let modulo_div = self.cycle % freq;
        if modulo_div == 0 {
            let tima = self.read(TIMA);
            let (res, overflow) = tima.overflowing_add(1);
            if overflow {
                let tma = self.read(TMA);
                self.write(TIMA, tma);
                self.interrupt.require_timer();
            } else {
                self.write(TIMA, res);
            }
        }
    }
}
