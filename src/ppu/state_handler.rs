use crate::ppu::Ppu;
use crate::ppu::State;
use crate::ppu::State::{Mode0, Mode1, Mode2, Mode3};

impl Ppu {
    pub fn update_state(&mut self) {
        println!(
            "mode: {:?}, dot {} stat: {:b}, x {}, y {}",
            self.state, self.dot, self.stat, self.x, self.ly
        );
        if self.state == Mode2 && self.dot % 456 >= 80 {
            self.state = Mode3;
            self.set_lcd_stat(Mode3);
        } else if self.state == Mode3 && self.dot % 456 >= 260 {
            if self.stat & 0b_0000_1000 > 0 {
                self.stat_int = true;
            }
            self.state = Mode0;
            self.set_lcd_stat(Mode0);
        } else if self.state == Mode0 && self.dot % 456 == 0 {
            self.x = 0;
            if self.ly < 143 {
                self.ly += 1;
                if self.stat & 0b_0010_0000 > 0 {
                    self.stat_int = true;
                }
                self.state = Mode2;
                self.set_lcd_stat(Mode2);
            } else {
                self.state = Mode1;
                self.set_lcd_stat(Mode1);
                if (self.stat & 0b_0001_0000) > 0 {
                    self.stat_int = true;
                }
                self.vblank = true;
            }
        } else if self.state == Mode1 && self.dot % 70224 == 0 {
            self.ly = 0;
            self.state = Mode2;
            self.set_lcd_stat(Mode2);
        }
    }

    fn set_lcd_stat(&mut self, new_state: State) {
        match new_state {
            Mode3 => {
                self.stat |= 0b11;
            }
            Mode1 => {
                self.stat |= 0b01;
                self.stat &= 0b1111_1101;
            }
            Mode2 => {
                self.stat |= 0b10;
                self.stat &= 0b1111_1110;
            }
            Mode0 => self.stat &= 0b1111_1100,
        }
    }

    pub fn run_ppu(&mut self) {
        self.check_lcy_y();
        match self.state {
            Mode0 => {}
            Mode1 => {
                if self.dot % 456 == 0 && self.ly < 154 {
                    self.ly += 1;
                }
            }
            Mode3 => {
                if self.x < 160 {
                    self.render();
                }
            }
            _ => {}
        }
    }

    fn check_lcy_y(&mut self) {
        if self.lyc == self.ly && (self.stat & 0b_1000_0000) > 0 {
            self.stat_int = true;
        }
    }
}
