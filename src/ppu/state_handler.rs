use crate::ppu::Ppu;
use crate::ppu::PpuInterrupt::{None, Stat, Vblank};
use crate::ppu::State;
use crate::ppu::State::{Mode0, Mode1, Mode2, Mode3};

use super::PpuInterrupt;

impl Ppu {
    pub fn update_state(&mut self) {
        self.dot += 4;
        if self.state == Mode2 && self.dot % 456 >= 80 {
            self.state = Mode3;
            self.set_lcd_stat(Mode3);
        } else if self.state == Mode3 && self.dot % 456 >= (80 + 172) {
            self.state = Mode0;
            self.set_lcd_stat(Mode0);
        } else if self.state == Mode0 && self.dot % 456 == 0 {
            if self.ly < 144 {
                self.state = Mode2;
                self.set_lcd_stat(Mode2);
            } else {
                self.state = Mode1;
                self.set_lcd_stat(Mode1);
                self.interrupt = Vblank
            }
        } else if self.state == Mode1 && self.dot % 456 == 0 && self.ly == 0 {
            self.state = Mode2;
            self.set_lcd_stat(Mode2);
        }
    }

    fn set_lcd_stat(&mut self, new_state: State) {
        let before_stat = self.stat & 0b_0000_0111;
        match new_state {
            Mode3 => self.stat &= 0b00,
            Mode1 => self.stat &= 0b01,
            Mode2 => self.stat &= 0b10,
            Mode0 => self.stat &= 0b11,
        }
        match self.stat {
            0b_0010_0000 => {
                if before_stat == 0 && (self.stat & 0b10) > 0 {
                    self.interrupt = PpuInterrupt::Stat;
                }
            }
            0b_0001_0000 => {
                if before_stat == 0 && (self.stat & 0b1) > 0 {
                    self.interrupt = PpuInterrupt::Stat;
                }
            }
            0b_0000_1000 => {
                if before_stat == 0 && (self.stat & 0b11) > 0 {
                    self.interrupt = PpuInterrupt::Stat;
                }
            }
            _ => {}
        }
    }

    pub fn run_ppu(&mut self) {
        self.check_lcy_y();
        match self.state {
            Mode0 => {
                if (self.dot + 4) % 456 == 0 {
                    self.ly += 1;
                }
            }
            Mode1 => {
                if (self.dot + 4) % 456 == 0 {
                    self.ly += 1;
                }
            }
            Mode3 => {
                self.x += 1;
            }
            _ => {}
        }
    }
    fn check_lcy_y(&mut self) {
        if self.lyc == self.ly {
            let before = self.stat & 0b_0000_0111;
            self.stat |= 0x4;
            if before == 0 && (self.stat & 0b0100_0000) > 0 {
                self.interrupt = PpuInterrupt::Stat;
            }
        }
    }
}
