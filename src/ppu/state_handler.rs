use crate::ppu::Ppu;
use crate::ppu::PpuInterrupt::{None, Stat, Vblank};
use crate::ppu::State;
use crate::ppu::State::{Mode0, Mode1, Mode2, Mode3};

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
            if self.y < 144 {
                self.state = Mode2;
                self.set_lcd_stat(Mode2);
            } else {
                self.state = Mode1;
                self.set_lcd_stat(Mode1);
                self.interrupt = Vblank
            }
        } else if self.state == Mode1 && self.dot % 456 == 0 && self.y == 0 {
            self.state = Mode2;
            self.set_lcd_stat(Mode2);
        }
    }

    fn set_lcd_stat(&mut self, new_state: State) {
        match new_state {
            Mode0 => self.stat &= 0b00,
            Mode1 => self.stat &= 0b01,
            Mode2 => self.stat &= 0b10,
            Mode3 => self.stat &= 0b11,
        }
        // TODO: handle stat interrupt
    }

    pub fn run_ppu(&mut self) {
        match self.state {
            Mode0 => {
                if (self.dot + 4) % 456 == 0 {
                    self.y += 1;
                }
            }
            Mode1 => {
                if (self.dot + 4) % 456 == 0 {
                    self.y += 1;
                }
            }
            Mode3 => {
                self.x += 1;
            }
            _ => {}
        }
    }
}
