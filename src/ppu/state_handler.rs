use crate::ppu::Ppu;
use crate::ppu::State;
use crate::ppu::State::{Mode0, Mode1, Mode2, Mode3};

impl Ppu {
    pub fn run_ppu(&mut self) {
        self.dot += 1;
        match self.state {
            Mode0 => {
                if self.dot % 456 == 0 {
                    self.scanline_drawn = false;
                    if self.ly < 143 {
                        self.increment_ly();
                        self.switch_state(Mode2);
                    } else {
                        self.switch_state(Mode1);
                        self.vblank = true;
                    }
                }
            }
            Mode1 => {
                if self.dot <= 70224 {
                    if self.dot % 456 == 0 {
                        self.increment_ly();
                    }
                } else {
                    self.ly = 0;
                    self.state = Mode2;
                }
            }
            Mode2 => {
                if self.dot % 456 >= 80 {
                    self.switch_state(Mode3);
                }
            }
            Mode3 => {
                if !self.scanline_drawn {
                    self.render();
                    self.scanline_drawn = true;
                }
                if self.dot % 456 >= 80 + 172 {
                    self.switch_state(Mode0);
                }
            }
        }
    }

    fn switch_state(&mut self, state: State) {
        self.update_stat(&state);
        self.state = state;
    }

    fn increment_ly(&mut self) {
        self.ly += 1;
        self.check_lcy_y();
    }

    fn check_lcy_y(&mut self) {
        if self.lyc == self.ly && (self.stat & 0b_1000_0000) > 0 {
            self.stat_int = true;
        }
    }
}
