use crate::ppu::Ppu;
use crate::ppu::State;
use crate::ppu::State::{Mode0, Mode1, Mode2, Mode3};

use super::config::HEIGHT;
use super::config::WIDTH;

impl Ppu {
    pub fn run_ppu(&mut self) {
        self.dot += 1;
        match self.state {
            Mode0 => {
                if self.dot % 456 == 0 {
                    self.scanline_drawn = false;
                    self.increment_ly();
                    if self.ly < 143 {
                        self.switch_state(Mode2);
                    } else {
                        self.frame_drawn = true;
                        self.switch_state(Mode1);
                        self.vblank = true;
                    }
                } else if self.dot < 50 {
                    self.switch_state(Mode2);
                }
            }
            Mode1 => {
                if self.dot <= 70224 {
                    if self.dot % 456 == 0 {
                        self.increment_ly();
                    }
                } else {
                    self.ly = 0;
                    self.window_ly = 0;
                    self.dot = 0;
                    self.state = Mode2;
                }
            }
            Mode2 => {
                if self.dot % 456 >= 80 {
                    self.objects.clear();
                    self.build_objects_list();
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
        if self.window_visible() {
            self.window_ly += 1;
        }
        self.ly += 1;
        self.check_lcy_y();
    }

    pub fn window_visible(&mut self) -> bool {
        self.is_window()
            && self.wx < WIDTH as u8 + 7
            && self.wy < HEIGHT as u8
            && self.wy <= self.ly
    }

    fn check_lcy_y(&mut self) {
        self.stat_int_ly = self.lyc == self.ly && (self.stat & 0b_0100_0000) > 0;
        if self.lyc == self.ly {
            self.stat |= 0b100;
        } else {
            self.stat &= !0b100;
        }
    }
}
