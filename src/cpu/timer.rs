use crate::memorybus::MemoryBus;

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

impl MemoryBus {
    pub fn handle_timer(&mut self) {
        if self.handle_div() {
            self.handle_tima();
        }
    }

    fn handle_div(&mut self) -> bool {
        let frequ = self.freq_bit();
        let mut div = self.div;
        let before = ((div & frequ) != 0) && self.is_tima_on();
        div = div.wrapping_add(4);
        self.div = div;
        if before && ((div & frequ) == 0) {
            return true;
        }
        false
    }

    fn freq_bit(&mut self) -> u16 {
        let tac = self.read(TAC);
        match tac & 0b11 {
            0b00 => 1 << 9,
            0b01 => 1 << 3,
            0b10 => 1 << 5,
            0b11 => 1 << 7,
            _ => panic!("TIMER: Wrong tac access"),
        }
    }

    fn is_tima_on(&mut self) -> bool {
        let tac = self.read(TAC);
        tac >> 2 & 1 == 1
    }

    fn handle_tima(&mut self) {
        let (res, overflow) = self.tima.overflowing_add(1);
        if overflow {
            self.tima = self.tma;
            self.interrupt.require_timer();
        } else {
            self.tima = res;
        }
    }
}
